use std::time::Duration;

use audionimbus::{
    Context, ContextSettings, DirectSimulationParameters, Occlusion, SimulationFlags,
    SimulationInputs, TransmissionParameters,
};
use bevy::prelude::*;
use itertools::izip;
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};

use crate::client::{player::{Model, Viewpoint}, AppState};

pub struct AudioPlugin;

pub const FRAME_SIZE: usize = 1024;
pub const SAMPLING_RATE: usize = 48000;
pub const NUM_CHANNELS: usize = 2;
pub const AMBISONICS_ORDER: usize = 2;
pub const AMBISONICS_NUM_CHANNELS: usize = (AMBISONICS_ORDER + 1).pow(2);
pub const GAIN_FACTOR_DIRECT: f32 = 1.0;
pub const GAIN_FACTOR_REFLECTIONS: f32 = 0.3;
pub const GAIN_FACTOR_REVERB: f32 = 0.1;

#[derive(Resource)]
pub(crate) struct Audio {
    pub context: audionimbus::Context,
    pub settings: audionimbus::AudioSettings,
    pub scene: audionimbus::Scene,
    pub simulator: audionimbus::Simulator<audionimbus::Direct, audionimbus::Reflections, ()>,
    pub hrtf: audionimbus::Hrtf,
    pub direct_effect: audionimbus::DirectEffect,
    pub reflection_effect: audionimbus::ReflectionEffect,
    pub reverb_effect: audionimbus::ReflectionEffect,
    pub ambisonics_encode_effect: audionimbus::AmbisonicsEncodeEffect,
    pub ambisonics_decode_effect: audionimbus::AmbisonicsDecodeEffect,
    pub sink: Sink,
    pub timer: Timer,
}

pub struct AudioFrame {
    position: usize,
    data: Vec<f32>,
    channels: u16,
}

impl AudioFrame {
    pub fn new(data: Vec<f32>, channels: u16) -> Self {
        Self {
            position: 0,
            data,
            channels,
        }
    }
}

impl Iterator for AudioFrame {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.data.len() {
            let sample = self.data[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for AudioFrame {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.data.len())
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        SAMPLING_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.data.len() as f32 / (SAMPLING_RATE as f32 * self.channels as f32),
        ))
    }
}

#[derive(Component, Debug)]
#[require(GlobalTransform)]
pub struct AudioSource {
    pub source: audionimbus::Source,
    pub data: Vec<audionimbus::Sample>, // Mono
    pub is_repeating: bool,
    pub position: usize,
}

#[derive(Resource)]
pub struct ListenerSource {
    // Special source used for reverb.
    pub source: audionimbus::Source,
}

impl AudioPlugin {
    fn process_frame(
        mut commands: Commands,
        query_character: Query<(&GlobalTransform, &Viewpoint), With<Model>>,
        mut query_audio_sources: Query<(Entity, &GlobalTransform, &mut AudioSource)>,
        time: Res<Time>,
        mut audio: ResMut<Audio>,
        mut listener_source: ResMut<ListenerSource>,
    ) {
        audio.timer.tick(time.delta());

        let (global_transform, viewpoint) = query_character.single().unwrap();

        let listener_position = global_transform.translation() + viewpoint.translation;

        let listener_orientation_right = viewpoint.rotation * Vec3::X;
        let listener_orientation_up = viewpoint.rotation * Vec3::Y;
        let listener_orientation_ahead = viewpoint.rotation * -Vec3::Z;

        let listener_orientation = audionimbus::CoordinateSystem {
            right: audionimbus::Vector3::new(
                listener_orientation_right.x,
                listener_orientation_right.y,
                listener_orientation_right.z,
            ),
            up: audionimbus::Vector3::new(
                listener_orientation_up.x,
                listener_orientation_up.y,
                listener_orientation_up.z,
            ),
            ahead: audionimbus::Vector3::new(
                listener_orientation_ahead.x,
                listener_orientation_ahead.y,
                listener_orientation_ahead.z,
            ),
            origin: audionimbus::Point::new(
                listener_position.x,
                listener_position.y,
                listener_position.z,
            ),
        };

        listener_source.source.set_inputs(
            SimulationFlags::REFLECTIONS,
            SimulationInputs {
                source: audionimbus::CoordinateSystem {
                    origin: audionimbus::Vector3::new(
                        listener_position.x,
                        listener_position.y,
                        listener_position.z,
                    ),
                    ..Default::default()
                },
                direct_simulation: Some(DirectSimulationParameters {
                    distance_attenuation: Some(audionimbus::DistanceAttenuationModel::Default),
                    air_absorption: Some(audionimbus::AirAbsorptionModel::Default),
                    directivity: Some(audionimbus::Directivity::default()),
                    occlusion: Some(Occlusion {
                        transmission: Some(TransmissionParameters {
                            num_transmission_rays: 8,
                        }),
                        algorithm: audionimbus::OcclusionAlgorithm::Raycast,
                    }),
                }),
                reflections_simulation: Some(
                    audionimbus::ReflectionsSimulationParameters::Convolution {
                        baked_data_identifier: None,
                    },
                ),
                pathing_simulation: None,
            },
        );

        let simulation_flags =
            audionimbus::SimulationFlags::DIRECT | audionimbus::SimulationFlags::REFLECTIONS;

        audio.simulator.set_shared_inputs(
            simulation_flags,
            &audionimbus::SimulationSharedInputs {
                listener: listener_orientation,
                num_rays: 2048,
                num_bounces: 8,
                duration: 2.0,
                order: AMBISONICS_ORDER,
                irradiance_min_distance: 1.0,
                pathing_visualization_callback: None,
            },
        );

        audio.simulator.run_direct();
        audio.simulator.run_reflections();

        let reverb_simulation_outputs = listener_source
            .source
            .get_outputs(audionimbus::SimulationFlags::REFLECTIONS);

        let reverb_effect_params = reverb_simulation_outputs.reflections();

        let times_finished_this_tick = audio.timer.times_finished_this_tick();

        for _ in 0..times_finished_this_tick {
            let mut deinterleaved_container = vec![0.0; FRAME_SIZE * NUM_CHANNELS];

            for (entity, source_global_transform, mut audio_source) in
                query_audio_sources.iter_mut()
            {
                let frame = if audio_source.is_repeating {
                    let frame: Vec<_> = (0..FRAME_SIZE)
                        .map(|i| {
                            audio_source.data[(audio_source.position + i) % audio_source.data.len()]
                        })
                        .collect();

                    audio_source.position =
                        (audio_source.position + FRAME_SIZE) % audio_source.data.len();

                    frame
                } else {
                    let frame = (0..FRAME_SIZE)
                        .map(|i| {
                            let idx = audio_source.position + i;

                            if idx < audio_source.data.len() {
                                audio_source.data[idx]
                            } else {
                                0.0
                            }
                        })
                        .collect();

                    audio_source.position += FRAME_SIZE;

                    if audio_source.position >= audio_source.data.len() {
                        commands.entity(entity).despawn();
                    }

                    frame
                };

                let source_position = source_global_transform.translation();

                audio_source.source.set_inputs(
                    simulation_flags,
                    audionimbus::SimulationInputs {
                        source: audionimbus::CoordinateSystem {
                            origin: audionimbus::Vector3::new(
                                source_position.x,
                                source_position.y,
                                source_position.z,
                            ),
                            ..Default::default()
                        },
                        direct_simulation: Some(audionimbus::DirectSimulationParameters {
                            distance_attenuation: Some(
                                audionimbus::DistanceAttenuationModel::Default,
                            ),
                            air_absorption: Some(audionimbus::AirAbsorptionModel::Default),
                            directivity: Some(audionimbus::Directivity::default()),
                            occlusion: Some(audionimbus::Occlusion {
                                transmission: Some(audionimbus::TransmissionParameters {
                                    num_transmission_rays: 8,
                                }),
                                algorithm: audionimbus::OcclusionAlgorithm::Raycast,
                            }),
                        }),
                        reflections_simulation: Some(
                            audionimbus::ReflectionsSimulationParameters::Convolution {
                                baked_data_identifier: None,
                            },
                        ),
                        pathing_simulation: None,
                    },
                );

                let simulation_outputs = audio_source.source.get_outputs(simulation_flags);

                let direct_effect_params = simulation_outputs.direct();

                let reflection_effect_params = simulation_outputs.reflections();

                let input_buffer = audionimbus::AudioBuffer::try_with_data(&frame).unwrap();

                let mut direct_container = vec![0.0; FRAME_SIZE];

                let direct_buffer =
                    audionimbus::AudioBuffer::try_with_data(&mut direct_container).unwrap();

                let _effect_state =
                    audio
                        .direct_effect
                        .apply(&direct_effect_params, &input_buffer, &direct_buffer);

                let direction = source_position - listener_position;

                let direction = audionimbus::Direction::new(direction.x, direction.y, direction.z);

                let mut ambisonics_encode_container =
                    vec![0.0; FRAME_SIZE * AMBISONICS_NUM_CHANNELS];

                let ambisonics_encode_buffer =
                    audionimbus::AudioBuffer::try_with_data_and_settings(
                        &mut ambisonics_encode_container,
                        &audionimbus::AudioBufferSettings {
                            num_channels: Some(AMBISONICS_NUM_CHANNELS),
                            ..Default::default()
                        },
                    )
                    .unwrap();

                let ambisonics_encode_effect_params = audionimbus::AmbisonicsEncodeEffectParams {
                    direction,
                    order: AMBISONICS_ORDER,
                };

                let _effect_state = audio.ambisonics_encode_effect.apply(
                    &ambisonics_encode_effect_params,
                    &direct_buffer,
                    &ambisonics_encode_buffer,
                );

                let mut reflection_container = vec![0.0; FRAME_SIZE * AMBISONICS_NUM_CHANNELS];

                let reflection_buffer = audionimbus::AudioBuffer::try_with_data_and_settings(
                    &mut reflection_container,
                    &audionimbus::AudioBufferSettings {
                        num_channels: Some(AMBISONICS_NUM_CHANNELS),
                        ..Default::default()
                    },
                )
                .unwrap();

                let _effect_state = audio.reflection_effect.apply(
                    &reflection_effect_params,
                    &input_buffer,
                    &reflection_buffer,
                );

                let mut reverb_container = vec![0.0; FRAME_SIZE * AMBISONICS_NUM_CHANNELS];

                let reverb_buffer = audionimbus::AudioBuffer::try_with_data_and_settings(
                    &mut reverb_container,
                    &audionimbus::AudioBufferSettings {
                        num_channels: Some(AMBISONICS_NUM_CHANNELS),
                        ..Default::default()
                    },
                )
                .unwrap();

                let _effect_state = audio.reverb_effect.apply(
                    &reverb_effect_params,
                    &input_buffer,
                    &reverb_buffer,
                );

                let mut mix_container = izip!(
                    ambisonics_encode_buffer.channels(),
                    reflection_buffer.channels(),
                    reverb_buffer.channels()
                )
                .flat_map(|(direct_channel, reflection_channel, reverb_channel)| {
                    izip!(
                        direct_channel.iter(),
                        reflection_channel.iter(),
                        reverb_channel.iter()
                    )
                    .map(
                        |(direct_sample, reflection_sample, reverb_sample)| {
                            (direct_sample * GAIN_FACTOR_DIRECT
                                + reflection_sample * GAIN_FACTOR_REFLECTIONS
                                + reverb_sample * GAIN_FACTOR_REVERB)
                                / (GAIN_FACTOR_DIRECT
                                    + GAIN_FACTOR_REFLECTIONS
                                    + GAIN_FACTOR_REVERB)
                        },
                    )
                })
                .collect::<Vec<_>>();

                let mix_buffer = audionimbus::AudioBuffer::try_with_data_and_settings(
                    &mut mix_container,
                    &audionimbus::AudioBufferSettings {
                        num_channels: Some(AMBISONICS_NUM_CHANNELS),
                        ..Default::default()
                    },
                )
                .unwrap();

                let mut staging_container = vec![0.0; FRAME_SIZE * NUM_CHANNELS];

                let staging_buffer = audionimbus::AudioBuffer::try_with_data_and_settings(
                    &mut staging_container,
                    &audionimbus::AudioBufferSettings {
                        num_channels: Some(NUM_CHANNELS),
                        ..Default::default()
                    },
                )
                .unwrap();

                let ambisonics_decode_effect_params = audionimbus::AmbisonicsDecodeEffectParams {
                    order: AMBISONICS_ORDER,
                    hrtf: &audio.hrtf,
                    orientation: listener_orientation,
                    binaural: false,
                };

                let _effect_state = audio.ambisonics_decode_effect.apply(
                    &ambisonics_decode_effect_params,
                    &mix_buffer,
                    &staging_buffer,
                );

                deinterleaved_container = staging_container
                    .iter()
                    .zip(deinterleaved_container.iter())
                    .map(|(a, b)| a + b)
                    .collect();

                let deinterleaved_buffer = audionimbus::AudioBuffer::try_with_data_and_settings(
                    &mut deinterleaved_container,
                    &audionimbus::AudioBufferSettings {
                        num_channels: Some(NUM_CHANNELS),
                        ..Default::default()
                    },
                )
                .unwrap();

                let mut interleaved = vec![0.0; FRAME_SIZE * NUM_CHANNELS];

                deinterleaved_buffer.interleave(&audio.context, &mut interleaved);

                let source = AudioFrame::new(interleaved, 2);

                audio.sink.append(source);
            }
        }
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();


        let sink = Sink::connect_new(&stream_handle.mixer());

        app.insert_non_send_resource(stream_handle);

        let context =
            audionimbus::Context::try_new(&audionimbus::ContextSettings::default()).unwrap();

        let settings = audionimbus::AudioSettings {
            frame_size: FRAME_SIZE,
            sampling_rate: SAMPLING_RATE,
        };

        let mut scene =
            audionimbus::Scene::try_new(&context, &audionimbus::SceneSettings::default()).unwrap();

        let mut simulator = audionimbus::Simulator::builder(
            audionimbus::SceneParams::Default,
            SAMPLING_RATE,
            FRAME_SIZE,
        )
        .with_direct(audionimbus::DirectSimulationSettings {
            max_num_occlusion_samples: 16,
        })
        .with_reflections(audionimbus::ReflectionsSimulationSettings::Convolution {
            max_num_rays: 2048,
            num_diffuse_samples: (8),
            max_duration: (2.0),
            max_order: (AMBISONICS_ORDER),
            max_num_sources: (8),
            num_threads: (1),
        })
        .try_build(&context)
        .unwrap();

        simulator.set_scene(&mut scene);

        let listener_source = audionimbus::Source::try_new(
            &simulator,
            &audionimbus::SourceSettings {
                flags: audionimbus::SimulationFlags::REFLECTIONS,
            },
        )
        .unwrap();

        simulator.add_source(&listener_source);

        app.insert_resource(ListenerSource {
            source: listener_source,
        });

        simulator.commit();

        let hrtf = audionimbus::hrtf::Hrtf::try_new(
            &context,
            &settings,
            &audionimbus::HrtfSettings {
                volume_normalization: audionimbus::VolumeNormalization::RootMeanSquared,
                ..Default::default()
            },
        )
        .unwrap();

        let direct_effect = audionimbus::DirectEffect::try_new(
            &context,
            &settings,
            &audionimbus::DirectEffectSettings { num_channels: 1 },
        )
        .unwrap();

        let reflection_effect = audionimbus::ReflectionEffect::try_new(
            &context,
            &settings,
            &audionimbus::ReflectionEffectSettings::Convolution {
                impulse_response_size: 2 * SAMPLING_RATE,
                num_channels: AMBISONICS_NUM_CHANNELS,
            },
        )
        .unwrap();

        let reverb_effect = audionimbus::ReflectionEffect::try_new(
            &context,
            &settings,
            &audionimbus::ReflectionEffectSettings::Convolution {
                impulse_response_size: (2 * SAMPLING_RATE),
                num_channels: AMBISONICS_NUM_CHANNELS,
            },
        )
        .unwrap();

        let ambisonics_encode_effect = audionimbus::AmbisonicsEncodeEffect::try_new(
            &context,
            &settings,
            &audionimbus::AmbisonicsEncodeEffectSettings {
                max_order: AMBISONICS_ORDER,
            },
        )
        .unwrap();

        let ambisonics_decode_effect = audionimbus::AmbisonicsDecodeEffect::try_new(
            &context,
            &settings,
            &audionimbus::AmbisonicsDecodeEffectSettings {
                max_order: AMBISONICS_ORDER,
                speaker_layout: audionimbus::SpeakerLayout::Stereo,
                hrtf: &hrtf,
            },
        )
        .unwrap();

        app.insert_resource(Audio {
            context,
            settings,
            scene,
            simulator,
            hrtf,
            direct_effect,
            reflection_effect,
            reverb_effect,
            ambisonics_decode_effect,
            ambisonics_encode_effect,
            sink,
            timer: Timer::new(
                Duration::from_secs_f32(FRAME_SIZE as f32 / SAMPLING_RATE as f32),
                TimerMode::Repeating,
            ),
        });

        app.add_systems(PostUpdate, Self::process_frame.run_if(in_state(AppState::InGame)));
    }
}

pub fn sine_wave(
    frequency: f32,
    sample_rate: usize,
    amplitude: f32,
    num_samples: usize,
) -> Vec<audionimbus::Sample> {
    let mut phase: f32 = 0.0;
    let phase_increment = 2.0 * std::f32::consts::PI * frequency / sample_rate as f32;
    (0..num_samples)
        .map(|_| {
            let sample = amplitude * phase.sin();
            phase = (phase + phase_increment) % (2.0 * std::f32::consts::PI);
            sample
        })
        .collect()
}
