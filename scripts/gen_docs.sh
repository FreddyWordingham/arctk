#!/bin/bash

cargo doc --all-features;
rm -r docs;
mv target/doc docs;
echo "<meta http-equiv=refresh content=0;url=arctk/index.html>" > docs/index.html;
