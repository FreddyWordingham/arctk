echo "Ch3: Babbage test I"
babbage output/tmp input/ ch3/babbage/gen_nano_map.json5;
mv output/tmp/nano.nc input/res/maps/;

echo "Ch4: Babbage test II"
babbage output/tmp input/ ch3/babbage/gen_point_map.json5;
mv output/tmp/point_50.nc input/res/maps/;

echo "Ch4: Cartographer test I"
cartographer output/ch3/cartographer/shape input/ ch3/cartographer/shape.json5

echo "Ch4: Diffuse test  I"
diffuse output/ch3/diffuse/kernel input/ ch3/diffuse/kernel.json5;

echo "Ch4: Diffuse test  II"
babbage output/tmp input/ ch3/babbage/gen_tenth_map.json5;
mv output/tmp/tenth.nc input/res/maps/;
babbage output/tmp input/ ch3/babbage/gen_cube_100_map.json5;
mv output/tmp/cube_100.nc input/res/maps/;
diffuse output/ch3/diffuse/cube input/ ch3/diffuse/cube.json5;

# read -rsp $'Flask Test: simple reaction\nPress any key to continue...' -n1 key;
# cargo run --bin flask --features="sim" output/ch3/flask/simple input/ ch3/flask/simple.json5;
# # flask output/ch3/flask/simple input/ ch3/flask/simple.json5;
# python3 input/plot.py output/ch3/flask/simple/values.csv

# read -rsp $'Flask Test: chain reaction\nPress any key to continue...' -n1 key;
# cargo run --bin flask --features="sim" output/ch3/flask/chain input/ ch3/flask/chain.json5;
# # flask output/ch3/flask/chain input/ ch3/flask/chain.json5;
# python3 input/plot.py output/ch3/flask/chain/values.csv

# read -rsp $'Flask Test: reversible reaction\nPress any key to continue...' -n1 key;
# cargo run --bin flask --features="sim" output/ch3/flask/equil input/ ch3/flask/equil.json5;
# # flask output/ch3/flask/equil input/ ch3/flask/equil.json5;
# python3 input/plot.py output/ch3/flask/equil/values.csv

# read -rsp $'Flask Test: source term\nPress any key to continue...' -n1 key;
# cargo run --bin flask --features="sim" output/ch3/flask/source input/ ch3/flask/source.json5;
# # flask output/ch3/flask/source input/ ch3/flask/source.json5;
# python3 input/plot.py output/ch3/flask/source/values.csv

# read -rsp $'Flask Test: sink term\nPress any key to continue...' -n1 key;
# cargo run --bin flask --features="sim" output/ch3/flask/sink input/ ch3/flask/sink.json5;
# # flask output/ch3/flask/sink input/ ch3/flask/sink.json5;
# python3 input/plot.py output/ch3/flask/sink/values.csv

# read -rsp $'Reactor Test: diffusion\nPress any key to continue...' -n1 key;
babbage output/tmp input/ ch3/babbage/gen_unit_101_map.json5;
mv output/tmp/unit_101.nc input/res/maps/;
reactor output/ch3/reactor/diffuse input/ ch3/reactor/diffuse.json5

# read -rsp $'Reactor Test: duel\nPress any key to continue...' -n1 key;
babbage output/tmp input/ ch3/babbage/gen_cube_101_map.json5;
mv output/tmp/cube_101.nc input/res/maps/;
reactor output/ch3/reactor/duel input/ ch3/reactor/duel.json5


# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ ch3/mcrt/absorption.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ ch3/mcrt/ccd.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ ch3/mcrt/imager.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ ch3/mcrt/photo.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ ch3/mcrt/spectrometer.json5;
