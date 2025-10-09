git ls-files "*.rs" "*.toml" | xargs wc -l > stats.txt
echo "" >> stats.txt
git log --shortstat --pretty=format:"" | awk "{ins+=\$4; del+=\$6} END {print \"Inserted:\", ins, \"Deleted:\", del, \"Total:\", ins+del}" >> stats.txt
