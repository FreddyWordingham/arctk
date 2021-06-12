# read -rsp $'Setup: oxygen diffusion\nPress any key to continue...' -n1 key;
echo "Ch4: Oxygen setup"
cartographer output/ch4/cartographer/skin input/ ch4/cartographer/skin.json5;
babbage output/tmp input/ ch4/babbage/build_map_diff_oxy.json5;
mv output/tmp/diff_oxy.nc input/res/maps/;
babbage output/tmp input/ ch4/babbage/build_map_source_oxy.json5;
mv output/tmp/source_oxy.nc input/res/maps/;
diffuse output/ch4/diffuse/oxy input/ ch4/diffuse/oxy.json5;
mv output/ch4/diffuse/oxy/015_diff.nc input/res/maps/init_oxy.nc;

echo "Ch4: ALA setup"
cartographer output/ch4/cartographer/cream input/ ch4/cartographer/cream.json5;
babbage output/tmp input/ ch4/babbage/build_map_init_ala.json5;
mv output/tmp/init_ala.nc input/res/maps/;
babbage output/tmp input/ ch4/babbage/build_map_diff_ala.json5;
mv output/tmp/diff_ala.nc input/res/maps/;


echo "Ch4: Shallow tumour setup"
cartographer output/ch4/cartographer/tumour/shallow input/ ch4/cartographer/tumour/shallow.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_shallow.json5;
mv output/tmp/multipliers_shallow.nc input/res/maps/;
mv output/ch4/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
reactor output/ch4/reactor/ppix/shallow input/ ch4/reactor/ppix/shallow.json5;
mv output/ch4/reactor/ppix/shallow/015_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc;
mv output/ch4/reactor/ppix/shallow/015_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc;

echo "Ch4: Shallow tumour run"
mcrt output/ch4/mcrt/shallow input/ ch4/mcrt/tumour/shallow.json5;
mv output/ch4/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
reactor output/ch4/reactor/pdt/shallow input/ ch4/reactor/pdt/shallow.json5;
babbage output/tmp input/ ch4/babbage/build_map_tumour_shallow_kill.json5;


echo "Ch4: Thick tumour setup"
cartographer output/ch4/cartographer/tumour/thick input/ ch4/cartographer/tumour/thick.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_thick.json5;
mv output/tmp/multipliers_thick.nc input/res/maps/;
mv output/ch4/cartographer/tumour/thick/map_\{tumour\}.nc input/res/maps/tumour_thick.nc;
reactor output/ch4/reactor/ppix/thick input/ ch4/reactor/ppix/thick.json5;
mv output/ch4/reactor/ppix/thick/015_\{ppix\}_diff.nc input/res/maps/init_ppix_thick.nc;
mv output/ch4/reactor/ppix/thick/015_\{ala\}_diff.nc input/res/maps/init_ala_thick.nc;

echo "Ch4: Thick tumour run"
mcrt output/ch4/mcrt/thick input/ ch4/mcrt/tumour/thick.json5;
mv output/ch4/mcrt/thick/shift_density.nc input/res/maps/udens_thick.nc;
reactor output/ch4/reactor/pdt/thick input/ ch4/reactor/pdt/thick.json5;
babbage output/tmp input/ ch4/babbage/build_map_tumour_thick_kill.json5;


echo "Ch4: Deep tumour setup"
cartographer output/ch4/cartographer/tumour/deep input/ ch4/cartographer/tumour/deep.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_deep.json5;
mv output/tmp/multipliers_deep.nc input/res/maps/;
mv output/ch4/cartographer/tumour/deep/map_\{tumour\}.nc input/res/maps/tumour_deep.nc;
reactor output/ch4/reactor/ppix/deep input/ ch4/reactor/ppix/deep.json5;
mv output/ch4/reactor/ppix/deep/015_\{ppix\}_diff.nc input/res/maps/init_ppix_deep.nc;
mv output/ch4/reactor/ppix/deep/015_\{ala\}_diff.nc input/res/maps/init_ala_deep.nc;

echo "Ch4: Deep tumour run"
mcrt output/ch4/mcrt/deep input/ ch4/mcrt/tumour/deep.json5;
mv output/ch4/mcrt/deep/shift_density.nc input/res/maps/udens_deep.nc;
reactor output/ch4/reactor/pdt/deep input/ ch4/reactor/pdt/deep.json5;
babbage output/tmp input/ ch4/babbage/build_map_tumour_deep_kill.json5;
