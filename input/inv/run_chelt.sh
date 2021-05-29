#   Chelt setup.
# read -rsp $'Setup: Chelating agent diffusion\nPress any key to continue...' -n1 key;
touch 4__Setup__chelt_cartography.txt;
cartographer output/inv/cartographer/chelt input/ inv/cartographer/chelt.json5;
babbage output/tmp input/ inv/babbage/build_map_init_chelt.json5;
mv output/tmp/init_chelt.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_diff_chelt.json5;
mv output/tmp/diff_chelt.nc input/res/maps/;


#   Shallow tumour
# read -rsp $'Shallow tumour: PpIX profile\nPress any key to continue...' -n1 key;
touch 3a_Setup__shallow_cartography.txt;
cartographer output/inv/cartographer/tumour/shallow input/ inv/cartographer/tumour/shallow.json5;
babbage output/tmp input/ inv/babbage/build_map_multipliers_tumour_shallow.json5;
mv output/tmp/multipliers_shallow.nc input/res/maps/;
mv output/inv/cartographer/tumour/shallow/map_\{tumour\}.nc input/res/maps/tumour/shallow.nc;
touch 3b_Setup__PPIX_generation.txt;
reactor output/inv/reactor/ppix/shallow input/ inv/reactor/ppix/shallow.json5;
mv output/inv/reactor/ppix/shallow/099_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc;
mv output/inv/reactor/ppix/shallow/099_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc;

#   PDT phase.
# read -rsp $'Shallow tumour: PDT phase\nPress any key to continue...' -n1 key;
touch 3c_Sim__illumination.txt;
mcrt output/inv/mcrt/shallow input/ inv/mcrt/tumour/shallow.json5;
mv output/inv/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
touch 3d_Sim__photodynamic_therapy.txt;
reactor output/inv/reactor/pdt/shallow input/ inv/reactor/pdt/shallow.json5;
babbage output/tmp input/ inv/babbage/build_map_tumour_shallow_kill.json5;
touch 3e_COMPLETE.txt;
