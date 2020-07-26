#!/bin/bash

cd $ARCTK_DIR;

cargo doc;
rm -r $ARCTK_DIR/docs;
mv $ARCTK_DIR/target/doc $ARCTK_DIR/docs;
echo "<meta http-equiv=refresh content=0;url=arctk/index.html>" > $ARCTK_DIR/docs/index.html;

cd -;
