# read -rsp $'Babbage Test: generate filled map\nPress any key to continue...' -n1 key;
# babbage output/tmp input/ test/babbage/gen_nano_map.json5;
# mv output/tmp/nano.nc input/res/maps/;

# read -rsp $'Babbage Test: generate point map\nPress any key to continue...' -n1 key;
# babbage output/tmp input/ test/babbage/gen_point_map.json5;
# mv output/tmp/point_50.nc input/res/maps/;

# read -rsp $'Cartographer Test: torus knot interior volume map\nPress any key to continue...' -n1 key;
# cartographer output/test/cartographer/shape input/ test/cartographer/shape.json5

# read -rsp $'Diffuse Test: heat kernel\nPress any key to continue...' -n1 key;
# diffuse output/test/diffuse/kernel input/ test/diffuse/kernel.json5;

read -rsp $'Diffuse Test: heat cube\nPress any key to continue...' -n1 key;
babbage output/tmp input/ test/babbage/gen_tenth_map.json5;
mv output/tenth.nc input/res/maps/;
babbage output/tmp input/ test/babbage/gen_cube_map.json5;
mv output/cube_100.nc input/res/maps/;
diffuse output/test/diffuse/cube input/ test/diffuse/cube.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# flask output input/ test/flask/chain.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ test/mcrt/absorption.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ test/mcrt/ccd.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ test/mcrt/imager.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ test/mcrt/photo.json5;

# read -rsp $'Press any key to continue...\n' -n1 key;
# mcrt output input/ test/mcrt/spectrometer.json5;
