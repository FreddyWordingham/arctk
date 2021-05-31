#   Oxygen setup.
# read -rsp $'Setup: oxygen diffusion\nPress any key to continue...' -n1 key;
touch output/ch4/0__Setup__oxygen_cartography.txt;
cartographer output/ch4/cartographer/skin input/ ch4/cartographer/skin.json5;
babbage output/tmp input/ ch4/babbage/build_map_diff_oxy.json5;
mv output/tmp/diff_oxy.nc input/res/maps/;
babbage output/tmp input/ ch4/babbage/build_map_source_oxy.json5;
mv output/tmp/source_oxy.nc input/res/maps/;
touch output/ch4/1__Setup__oxygen_diffusion.txt;
diffuse output/ch4/diffuse/oxy input/ ch4/diffuse/oxy.json5;
mv output/ch4/diffuse/oxy/099_diff.nc input/res/maps/init_oxy.nc;

#   ALA setup.
# read -rsp $'Setup: ALA diffusion\nPress any key to continue...' -n1 key;
touch output/ch4/2__Setup__ALA_cartography.txt;
cartographer output/ch4/cartographer/cream input/ ch4/cartographer/cream.json5;
babbage output/tmp input/ ch4/babbage/build_map_init_ala.json5;
mv output/tmp/init_ala.nc input/res/maps/;
babbage output/tmp input/ ch4/babbage/build_map_diff_ala.json5;
mv output/tmp/diff_ala.nc input/res/maps/;



#   Shallow tumour
# read -rsp $'Shallow tumour: PpIX profile\nPress any key to continue...' -n1 key;
touch output/ch4/3a_Setup__shallow_cartography.txt;
cartographer output/ch4/cartographer/tumour/shallow input/ ch4/cartographer/tumour/shallow.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_shallow.json5;
mv output/tmp/multipliers_shallow.nc input/res/maps/;
mv output/ch4/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
touch output/ch4/3b_Setup__PPIX_generation.txt;
reactor output/ch4/reactor/ppix/shallow input/ ch4/reactor/ppix/shallow.json5;
mv output/ch4/reactor/ppix/shallow/099_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc;
mv output/ch4/reactor/ppix/shallow/099_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc;

#   PDT phase.
# read -rsp $'Shallow tumour: PDT phase\nPress any key to continue...' -n1 key;
touch output/ch4/3c_Sim__illumination.txt;
mcrt output/ch4/mcrt/shallow input/ ch4/mcrt/tumour/shallow.json5;
mv output/ch4/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
touch output/ch4/3d_Sim__photodynamic_therapy.txt;
reactor output/ch4/reactor/pdt/shallow input/ ch4/reactor/pdt/shallow.json5;
mv output/ch4/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
babbage output/tmp input/ ch4/babbage/build_map_tumour_shallow_kill.json5;
touch output/ch4/3e_COMPLETE.txt;



#   Thick tumour
# read -rsp $'Thick tumour: PpIX profile\nPress any key to continue...' -n1 key;
touch output/ch4/4a_Setup__thick_cartography.txt;
cartographer output/ch4/cartographer/tumour/thick input/ ch4/cartographer/tumour/thick.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_thick.json5;
mv output/tmp/multipliers_thick.nc input/res/maps/;
mv output/ch4/cartographer/tumour/thick/map_\{tumour\}.nc input/res/maps/tumour_thick.nc;
touch output/ch4/4b_Setup__PPIX_generation.txt;
reactor output/ch4/reactor/ppix/thick input/ ch4/reactor/ppix/thick.json5;
mv output/ch4/reactor/ppix/thick/099_\{ppix\}_diff.nc input/res/maps/init_ppix_thick.nc;
mv output/ch4/reactor/ppix/thick/099_\{ala\}_diff.nc input/res/maps/init_ala_thick.nc;

#   PDT phase.
# read -rsp $'Thick tumour: PDT phase\nPress any key to continue...' -n1 key;
touch output/ch4/4c_Sim__illumination.txt;
mcrt output/ch4/mcrt/thick input/ ch4/mcrt/tumour/thick.json5;
mv output/ch4/mcrt/thick/shift_density.nc input/res/maps/udens_thick.nc;
touch output/ch4/4d_Sim__photodynamic_therapy.txt;
reactor output/ch4/reactor/pdt/thick input/ ch4/reactor/pdt/thick.json5;
babbage output/tmp input/ ch4/babbage/build_map_tumour_thick_kill.json5;
touch output/ch4/4e_COMPLETE.txt;



#   Deep tumour
# read -rsp $'Deep tumour: PpIX profile\nPress any key to continue...' -n1 key;
touch output/ch4/5a_Setup__deep_cartography.txt;
cartographer output/ch4/cartographer/tumour/deep input/ ch4/cartographer/tumour/deep.json5;
babbage output/tmp input/ ch4/babbage/build_map_multipliers_tumour_deep.json5;
mv output/tmp/multipliers_deep.nc input/res/maps/;
mv output/ch4/cartographer/tumour/deep/map_\{tumour\}.nc input/res/maps/tumour_deep.nc;
touch output/ch4/5b_Setup__PPIX_generation.txt;
reactor output/ch4/reactor/ppix/deep input/ ch4/reactor/ppix/deep.json5;
mv output/ch4/reactor/ppix/deep/099_\{ppix\}_diff.nc input/res/maps/init_ppix_deep.nc;
mv output/ch4/reactor/ppix/deep/099_\{ala\}_diff.nc input/res/maps/init_ala_deep.nc;

#   PDT phase.
# read -rsp $'Deep tumour: PDT phase\nPress any key to continue...' -n1 key;
touch output/ch4/5c_Sim__illumination.txt;
mcrt output/ch4/mcrt/deep input/ ch4/mcrt/tumour/deep.json5;
mv output/ch4/mcrt/deep/shift_density.nc input/res/maps/udens_deep.nc;
touch output/ch4/5d_Sim__photodynamic_therapy.txt;
reactor output/ch4/reactor/pdt/deep input/ ch4/reactor/pdt/deep.json5;
babbage output/tmp input/ ch4/babbage/build_map_tumour_deep_kill.json5;
touch output/ch4/5e_COMPLETE.txt;
