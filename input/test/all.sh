read -rsp $'Press any key to continue...\n' -n1 key;
babbage output input/ test/babbage/gen_nano_map.json5;
mv output/nano.nc input/res/maps/;

read -rsp $'Press any key to continue...\n' -n1 key;
babbage output input/ test/babbage/gen_point_map.json5;
mv output/point_50.nc input/res/maps/;

read -rsp $'Press any key to continue...\n' -n1 key;
cartographer output input/ test/cartographer/shape.json5

read -rsp $'Press any key to continue...\n' -n1 key;
diffuse output input/ test/diffuse/point.json5;

read -rsp $'Press any key to continue...\n' -n1 key;
babbage output input/ test/babbage/gen_tenth_map.json5;
mv output/tenth.nc input/res/maps/;
babbage output input/ test/babbage/gen_cube_map.json5;
mv output/cube_100.nc input/res/maps/;
diffuse output input/ test/diffuse/cube.json5;

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
