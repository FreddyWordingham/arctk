echo "Ch3: Babbage test I"
babbage output/tmp input/ ch3/babbage/gen_nano_map.json5;
mv output/tmp/nano.nc input/res/maps/;

echo "Ch3: Babbage test II"
babbage output/tmp input/ ch3/babbage/gen_point_101_map.json5;
mv output/tmp/point_101.nc input/res/maps/;


echo "Ch3: Cartographer test I"
cartographer output/ch3/cartographer/shape input/ ch3/cartographer/shape.json5;


echo "Ch3: Diffuse test I"
diffuse output/ch3/diffuse/kernel input/ ch3/diffuse/kernel.json5;

echo "Ch3: Diffuse test II"
babbage output/tmp input/ ch3/babbage/gen_tenth_map.json5;
mv output/tmp/tenth.nc input/res/maps/;
babbage output/tmp input/ ch3/babbage/gen_cube_100_map.json5;
mv output/tmp/cube_100.nc input/res/maps/;
diffuse output/ch3/diffuse/cube input/ ch3/diffuse/cube.json5;


echo "Ch3: Flask test I"
flask output/ch3/flask/simple input/ ch3/flask/simple.json5;
# python3 input/plot.py output/ch3/flask/simple/values.csv

echo "Ch3: Flask test II"
flask output/ch3/flask/chain input/ ch3/flask/chain.json5;
# python3 input/plot.py output/ch3/flask/chain/values.csv

echo "Ch3: Flask test III"
flask output/ch3/flask/equilibrium input/ ch3/flask/equilibrium.json5;
# python3 input/plot.py output/ch3/flask/chain/values.csv

echo "Ch3: Flask test IV"
flask output/ch3/flask/source input/ ch3/flask/source.json5;
# python3 input/plot.py output/ch3/flask/source/values.csv

echo "Ch3: Flask test V"
flask output/ch3/flask/sink input/ ch3/flask/sink.json5;
# python3 input/plot.py output/ch3/flask/sink/values.csv


echo "Ch3: Reactor test I"
babbage output/tmp input/ ch3/babbage/gen_unit_51_map.json5;
mv output/tmp/unit_51.nc input/res/maps/;
babbage output/tmp input/ ch3/babbage/gen_point_51_map.json5;
mv output/tmp/point_51.nc input/res/maps/;
reactor output/ch3/reactor/diffuse input/ ch3/reactor/diffuse.json5;

echo "Ch3: Reactor test II"
babbage output/tmp input/ ch3/babbage/gen_cube_51_map.json5;
mv output/tmp/cube_51.nc input/res/maps/;
reactor output/ch3/reactor/react input/ ch3/reactor/react.json5;

echo "Ch3: Reactor test III"
reactor output/ch3/reactor/coupled input/ ch3/reactor/coupled.json5;


echo "Ch3: MCRT test I"
mcrt output/ch3/mcrt/prism input/ ch3/mcrt/prism.json5;

echo "Ch3: MCRT test II"
mcrt output/ch3/mcrt/rainbow input/ ch3/mcrt/rainbow.json5;

echo "Ch3: MCRT test III"
mcrt output/ch3/mcrt/lens input/ ch3/mcrt/lens.json5;

echo "Ch3: MCRT test IV"
mcrt output/ch3/mcrt/fibre input/ ch3/mcrt/fibre.json5;

echo "Ch3: MCRT test V"
mcrt output/ch3/mcrt/absorption input/ ch3/mcrt/absorption.json5;

echo "Ch3: MCRT test VI"
mcrt output/ch3/mcrt/beam input/ ch3/mcrt/beam.json5;

echo "Ch3: MCRT test VII"
mcrt output/ch3/mcrt/scatter input/ ch3/mcrt/scatter.json5;

echo "Ch3: MCRT test VIII"
mcrt output/ch3/mcrt/ccd input/ ch3/mcrt/ccd.json5;

echo "Ch3: MCRT test IX"
mcrt output/ch3/mcrt/spectrometer input/ ch3/mcrt/spectrometer.json5;
# python3 input/plot.py output/ch3/mcrt/spectrometer/spectrometer_\{spectrometer\}.csv

echo "Ch3: MCRT test X"
mcrt output/ch3/mcrt/photo input/ ch3/mcrt/photo.json5;


echo "Ch3: Render test I"
render output/ch3/render/dream input/ ch3/render/dream.json5;
