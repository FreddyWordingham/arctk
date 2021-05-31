#   Fibre PDT.
# read -rsp $'Fibre tumour\nPress any key to continue...' -n1 key;
touch output/ch7/3a_Sim__illumination.txt;
mcrt output/ch7/mcrt/fibre input/ ch7/mcrt/fibre.json5;
mv output/ch7/mcrt/fibre/shift_density.nc input/res/maps/udens_fibre.nc;
touch output/ch7/3b_Sim__photodynamic_therapy.txt;
reactor output/ch7/reactor/fibre input/ ch7/reactor/pdt/fibre.json5;
babbage output/tmp input/ ch7/babbage/build_map_tumour_fibre_kill.json5;
touch output/ch7/3c_COMPLETE.txt;



#   Fibres PDT.
# read -rsp $'Fibres tumour\nPress any key to continue...' -n1 key;
touch output/ch7/4a_Sim__illumination.txt;
mcrt output/ch7/mcrt/fibres input/ ch7/mcrt/fibres.json5;
mv output/ch7/mcrt/fibres/shift_density.nc input/res/maps/udens_fibres.nc;
touch output/ch7/4b_Sim__photodynamic_therapy.txt;
reactor output/ch7/reactor/fibres input/ ch7/reactor/pdt/fibres.json5;
babbage output/tmp input/ ch7/babbage/build_map_tumour_fibres_kill.json5;
touch output/ch7/4c_COMPLETE.txt;



#   Chelt setup.
# read -rsp $'Setup: Chelating agent diffusion\nPress any key to continue...' -n1 key;
touch 4__Setup__chelt_cartography.txt;
cartographer output/inv/cartographer/chelt input/ inv/cartographer/chelt.json5;
babbage output/tmp input/ inv/babbage/build_map_init_fe.json5;
mv output/tmp/init_fe.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_init_chelt.json5;
mv output/tmp/init_chelt.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_diff_chelt.json5;
mv output/tmp/diff_chelt.nc input/res/maps/;





#   Shallow tumour
# read -rsp $'Shallow tumour: Chelated PpIX profile\nPress any key to continue...' -n1 key;
touch 5a_Setup__Chelt_PPIX_generation.txt;
reactor output/inv/reactor/haem/shallow input/ inv/reactor/haem/shallow.json5;
mv output/inv/reactor/haem/shallow/099_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc;
mv output/inv/reactor/haem/shallow/099_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc;
mv output/inv/reactor/haem/shallow/099_\{fe\}_diff.nc input/res/maps/init_fe_shallow.nc;
mv output/inv/reactor/haem/shallow/099_\{chelt\}_diff.nc input/res/maps/init_haem_shallow.nc;
mv output/inv/reactor/haem/shallow/099_\{haem\}_diff.nc input/res/maps/init_haem_shallow.nc;

# #   PDT phase.
# # read -rsp $'Shallow tumour: PDT phase\nPress any key to continue...' -n1 key;
# touch 3c_Sim__illumination.txt;
# mcrt output/inv/mcrt/shallow input/ inv/mcrt/tumour/shallow.json5;
# mv output/inv/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
# touch 3d_Sim__photodynamic_therapy.txt;
# reactor output/inv/reactor/pdt/shallow input/ inv/reactor/pdt/shallow.json5;
# babbage output/tmp input/ inv/babbage/build_map_tumour_shallow_kill.json5;
# touch 3e_COMPLETE.txt;
