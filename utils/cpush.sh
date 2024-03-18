
# commit & push script

git add .
msg="$@"
git commit -am "$msg"
git push
