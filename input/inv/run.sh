#   Oxygen setup.
# read -rsp $'Setup: oxygen diffusion\nPress any key to continue...' -n1 key;
touch 0__Setup__oxygen_cartography.txt;
cartographer output/inv/cartographer/skin input/ inv/cartographer/skin.json5;
babbage output/tmp input/ inv/babbage/build_map_diff_oxy.json5;
mv output/tmp/diff_oxy.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_source_oxy.json5;
mv output/tmp/source_oxy.nc input/res/maps/;
touch 1__Setup__oxygen_diffusion.txt;
diffuse output/inv/diffuse/oxy input/ inv/diffuse/oxy.json5;
mv output/inv/diffuse/oxy/099_diff.nc input/res/maps/init_oxy.nc;

#   ALA setup.
# read -rsp $'Setup: ALA diffusion\nPress any key to continue...' -n1 key;
touch 2__Setup__ALA_cartography.txt;
cartographer output/inv/cartographer/cream input/ inv/cartographer/cream.json5;
babbage output/tmp input/ inv/babbage/build_map_init_ala.json5;
mv output/tmp/init_ala.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_diff_ala.json5;
mv output/tmp/diff_ala.nc input/res/maps/;



# #   Shallow tumour
# # read -rsp $'Shallow tumour: PpIX profile\nPress any key to continue...' -n1 key;
# touch 3a_Setup__shallow_cartography.txt;
# cartographer output/inv/cartographer/tumour/shallow input/ inv/cartographer/tumour/shallow.json5;
# babbage output/tmp input/ inv/babbage/build_map_multipliers_tumour_shallow.json5;
# mv output/tmp/multipliers_shallow.nc input/res/maps/;
# mv output/inv/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
# touch 3b_Setup__PPIX_generation.txt;
# reactor output/inv/reactor/ppix/shallow input/ inv/reactor/ppix/shallow.json5;
# mv output/inv/reactor/ppix/shallow/099_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc;
# mv output/inv/reactor/ppix/shallow/099_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc;

# #   PDT phase.
# # read -rsp $'Shallow tumour: PDT phase\nPress any key to continue...' -n1 key;
# touch 3c_Sim__illumination.txt;
# mcrt output/inv/mcrt/shallow input/ inv/mcrt/tumour/shallow.json5;
# mv output/inv/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
# touch 3d_Sim__photodynamic_therapy.txt;
# reactor output/inv/reactor/pdt/shallow input/ inv/reactor/pdt/shallow.json5;
# mv output/inv/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
# babbage output/tmp input/ inv/babbage/build_map_tumour_shallow_kill.json5;
# touch 3e_COMPLETE.txt;



#   Thick tumour
# read -rsp $'Thick tumour: PpIX profile\nPress any key to continue...' -n1 key;
touch 4a_Setup__thick_cartography.txt;
cartographer output/inv/cartographer/tumour/thick input/ inv/cartographer/tumour/thick.json5;
babbage output/tmp input/ inv/babbage/build_map_multipliers_tumour_thick.json5;
mv output/tmp/multipliers_thick.nc input/res/maps/;
mv output/inv/cartographer/tumour/thick/map_\{tumour\}.nc input/res/maps/tumour_thick.nc;
touch 4b_Setup__PPIX_generation.txt;
reactor output/inv/reactor/ppix/thick input/ inv/reactor/ppix/thick.json5;
mv output/inv/reactor/ppix/thick/099_\{ppix\}_diff.nc input/res/maps/init_ppix_thick.nc;
mv output/inv/reactor/ppix/thick/099_\{ala\}_diff.nc input/res/maps/init_ala_thick.nc;

#   PDT phase.
# read -rsp $'Thick tumour: PDT phase\nPress any key to continue...' -n1 key;
touch 4c_Sim__illumination.txt;
mcrt output/inv/mcrt/thick input/ inv/mcrt/tumour/thick.json5;
mv output/inv/mcrt/thick/shift_density.nc input/res/maps/udens_thick.nc;
touch 4d_Sim__photodynamic_therapy.txt;
reactor output/inv/reactor/pdt/thick input/ inv/reactor/pdt/thick.json5;
babbage output/tmp input/ inv/babbage/build_map_tumour_thick_kill.json5;
touch 4e_COMPLETE.txt;



# #   Deep tumour
# # read -rsp $'Deep tumour: PpIX profile\nPress any key to continue...' -n1 key;
# touch 5a_Setup__deep_cartography.txt;
# cartographer output/inv/cartographer/tumour/deep input/ inv/cartographer/tumour/deep.json5;
# babbage output/tmp input/ inv/babbage/build_map_multipliers_tumour_deep.json5;
# mv output/tmp/multipliers_deep.nc input/res/maps/;
# mv output/inv/cartographer/tumour/deep/map_\{tumour\}.nc input/res/maps/tumour_deep.nc;
# touch 5b_Setup__PPIX_generation.txt;
# reactor output/inv/reactor/ppix/deep input/ inv/reactor/ppix/deep.json5;
# mv output/inv/reactor/ppix/deep/099_\{ppix\}_diff.nc input/res/maps/init_ppix_deep.nc;
# mv output/inv/reactor/ppix/deep/099_\{ala\}_diff.nc input/res/maps/init_ala_deep.nc;

# #   PDT phase.
# # read -rsp $'Deep tumour: PDT phase\nPress any key to continue...' -n1 key;
# touch 5c_Sim__illumination.txt;
# mcrt output/inv/mcrt/deep input/ inv/mcrt/tumour/deep.json5;
# mv output/inv/mcrt/deep/shift_density.nc input/res/maps/udens_deep.nc;
# touch 5d_Sim__photodynamic_therapy.txt;
# reactor output/inv/reactor/pdt/deep input/ inv/reactor/pdt/deep.json5;
# babbage output/tmp input/ inv/babbage/build_map_tumour_deep_kill.json5;
# touch 5e_COMPLETE.txt;
