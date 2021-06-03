# echo "Ch6: Spectrometer"
# mcrt output/ch6/mcrt/spectrometer input/ ch6/mcrt/spectrometer.json5;


# echo "Ch6: Spectrometer"
# mcrt output/ch6/mcrt/laser input/ ch6/mcrt/laser.json5;


# echo "Ch6: Fibre"
# mcrt output/ch6/mcrt/fibre input/ ch6/mcrt/fibre.json5;
# mv output/ch6/mcrt/fibre/shift_density.nc input/res/maps/udens_fibre.nc;
# reactor output/ch6/reactor/fibre input/ ch6/reactor/pdt/fibre.json5;
# babbage output/tmp input/ ch6/babbage/build_map_tumour_fibre_kill.json5;


# echo "Ch6: Fibres"
# mcrt output/ch6/mcrt/fibres input/ ch6/mcrt/fibres.json5;
# mv output/ch6/mcrt/fibres/shift_density.nc input/res/maps/udens_fibres.nc;
# reactor output/ch6/reactor/fibres input/ ch6/reactor/pdt/fibres.json5;
# babbage output/tmp input/ ch6/babbage/build_map_tumour_fibres_kill.json5;


echo "Ch6: Chelation"
cartographer output/ch6/cartographer/chelt input/ ch6/cartographer/chelt.json5;
babbage output/tmp input/ ch6/babbage/build_map_init_fe.json5;
mv output/tmp/init_fe.nc input/res/maps/;
babbage output/tmp input/ ch6/babbage/build_map_init_chelt.json5;
mv output/tmp/init_chelt.nc input/res/maps/;
babbage output/tmp input/ ch6/babbage/build_map_diff_chelt.json5;
mv output/tmp/diff_chelt.nc input/res/maps/;
reactor output/ch6/reactor/ppix/haem input/ ch6/reactor/ppix/haem.json5;
mv output/ch6/reactor/ppix/haem/099_\{ala\}_diff.nc input/res/maps/init_ala_chelt.nc;
mv output/ch6/reactor/ppix/haem/099_\{ppix\}_diff.nc input/res/maps/init_ppix_chelt.nc;
mv output/ch6/reactor/ppix/haem/099_\{chelt\}_diff.nc input/res/maps/init_chelt_chelt.nc;
mv output/ch6/reactor/ppix/haem/099_\{fe\}_diff.nc input/res/maps/init_fe_chelt.nc;
reactor output/ch6/reactor/chelt input/ ch6/reactor/chelt.json5;
