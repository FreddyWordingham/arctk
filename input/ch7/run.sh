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
touch output/ch7/5a_Setup__chelt_cartography.txt;
cartographer output/ch7/cartographer/chelt input/ ch7/cartographer/chelt.json5;
babbage output/tmp input/ ch7/babbage/build_map_init_fe.json5;
mv output/tmp/init_fe.nc input/res/maps/;
babbage output/tmp input/ ch7/babbage/build_map_init_chelt.json5;
mv output/tmp/init_chelt.nc input/res/maps/;
babbage output/tmp input/ ch7/babbage/build_map_diff_chelt.json5;
mv output/tmp/diff_chelt.nc input/res/maps/;
touch output/ch7/5b_Chelation.txt;
reactor output/ch7/reactor/ppix/haem input/ ch7/reactor/ppix/haem.json5;
mv output/ch7/reactor/ppix/haem/099_\{ala\}_diff.nc input/res/maps/init_ala_chelt.nc;
mv output/ch7/reactor/ppix/haem/099_\{ppix\}_diff.nc input/res/maps/init_ppix_chelt.nc;
mv output/ch7/reactor/ppix/haem/099_\{chelt\}_diff.nc input/res/maps/init_chelt_chelt.nc;
mv output/ch7/reactor/ppix/haem/099_\{fe\}_diff.nc input/res/maps/init_fe_chelt.nc;
touch output/ch7/5c_Chelt_PDT.txt;
reactor output/ch7/reactor/chelt input/ ch7/reactor/chelt.json5;
