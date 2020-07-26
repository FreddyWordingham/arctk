#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    exit;
fi

cd $ARCTK_DIR/output/render/$1;

echo "Composing normal";
composite -gravity center first_hit_edges_bk.png first_hit_norm.png normal.png;
composite -gravity center first_hit_norm_bk.png normal.png normal.png;

echo "Composing outline";
composite -gravity center first_hit_edges_bk.png first_hit_norm_bk.png outline.png;
convert -flatten outline.png outline.png;
convert outline.png -channel RGB -negate outline.png;

echo "Composing comic";
composite -gravity center first_hit_edges_bk.png image.png comic.png;
composite -gravity center first_hit_norm_bk.png comic.png comic.png;

cd -;
