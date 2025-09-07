#/bin/zsh
./build.sh
mkdir -p ../packages/packages/preview/comitia/0.1.0
cp -r ./typst-package/* ../packages/packages/preview/comitia/0.1.0/
cp -r ./LICENSE ../packages/packages/preview/comitia/0.1.0/
cp -r ./README.md ../packages/packages/preview/comitia/0.1.0/
sed -i '' 's|#import "./lib.typ"|#import "@preview/comitia:0.1.0"|' ../packages/packages/preview/comitia/0.1.0/example.typ
mv ../packages/packages/preview/comitia/0.1.0/example.typ ../packages/packages/preview/comitia/0.1.0/examples/example.typ