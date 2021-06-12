echo "Ch6: Spectrometer"
mcrt output/ch6/mcrt/spectrometer input/ ch6/mcrt/spectrometer.json5;


echo "Ch6: Laser"
mcrt output/ch6/mcrt/laser input/ ch6/mcrt/laser.json5;


echo "Ch6: Fibre"
mcrt output/ch6/mcrt/fibre input/ ch6/mcrt/fibre.json5;
cp output/ch6/mcrt/fibre/shift_density.nc input/res/maps/udens_fibre.nc;
reactor output/ch6/reactor/fibre input/ ch6/reactor/pdt/fibre.json5;
babbage output/tmp input/ ch6/babbage/build_map_tumour_fibre_kill.json5;


echo "Ch6: Fibres"
mcrt output/ch6/mcrt/fibres input/ ch6/mcrt/fibres.json5;
cp output/ch6/mcrt/fibres/shift_density.nc input/res/maps/udens_fibres.nc;
reactor output/ch6/reactor/fibres input/ ch6/reactor/pdt/fibres.json5;
babbage output/tmp input/ ch6/babbage/build_map_tumour_fibres_kill.json5;


echo "Ch6: Chelation"
cartographer output/ch6/cartographer/chelt input/ ch6/cartographer/chelt.json5;
babbage output/tmp input/ ch6/babbage/build_map_init_fe.json5;
cp output/tmp/init_fe.nc input/res/maps/;
babbage output/tmp input/ ch6/babbage/build_map_init_chelt.json5;
cp output/tmp/init_chelt.nc input/res/maps/;
babbage output/tmp input/ ch6/babbage/build_map_diff_chelt.json5;
cp output/tmp/diff_chelt.nc input/res/maps/;
reactor output/ch6/reactor/ppix/chelt input/ ch6/reactor/ppix/chelt.json5;
cp output/ch6/reactor/ppix/chelt/015_\{ala\}_diff.nc input/res/maps/init_ala_chelt.nc;
cp output/ch6/reactor/ppix/chelt/015_\{ppix\}_diff.nc input/res/maps/init_ppix_chelt.nc;
cp output/ch6/reactor/ppix/chelt/015_\{chelt\}_diff.nc input/res/maps/init_chelt_chelt.nc;
cp output/ch6/reactor/ppix/chelt/015_\{fe\}_diff.nc input/res/maps/init_fe_chelt.nc;
mcrt output/ch6/mcrt/chelt input/ ch6/mcrt/chelt.json5;
cp output/ch6/mcrt/chelt/shift_density.nc input/res/maps/udens_chelt.nc;
reactor output/ch6/reactor/pdt/chelt input/ ch6/reactor/pdt/chelt.json5;


echo "Ch6: Chelation"
babbage output/tmp input/ ch6/babbage/build_map_init_fe.json5;
cp output/tmp/init_fe.nc input/res/maps/;
reactor output/ch6/reactor/ppix/chelt_no input/ ch6/reactor/ppix/chelt_no.json5;
cp output/ch6/reactor/ppix/chelt_no/015_\{ala\}_diff.nc input/res/maps/init_ala_chelt_no.nc;
cp output/ch6/reactor/ppix/chelt_no/015_\{ppix\}_diff.nc input/res/maps/init_ppix_chelt_no.nc;
cp output/ch6/reactor/ppix/chelt_no/015_\{chelt\}_diff.nc input/res/maps/init_chelt_chelt_no.nc;
cp output/ch6/reactor/ppix/chelt_no/015_\{fe\}_diff.nc input/res/maps/init_fe_chelt_no.nc;
mcrt output/ch6/mcrt/chelt_no input/ ch6/mcrt/chelt_no.json5;
cp output/ch6/mcrt/chelt_no/shift_density.nc input/res/maps/udens_chelt_no.nc;
reactor output/ch6/reactor/pdt/chelt_no input/ ch6/reactor/pdt/chelt_no.json5;


# paste -d, stripe_000.csv <(cut -d"," -f2- stripe_001.csv) <(cut -d"," -f2- stripe_002.csv) <(cut -d"," -f2- stripe_003.csv) <(cut -d"," -f2- stripe_004.csv) <(cut -d"," -f2- stripe_005.csv) \
#     <(cut -d"," -f2- stripe_006.csv) <(cut -d"," -f2- stripe_007.csv) <(cut -d"," -f2- stripe_008.csv) <(cut -d"," -f2- stripe_009.csv) <(cut -d"," -f2- stripe_010.csv) \
#     <(cut -d"," -f2- stripe_011.csv) <(cut -d"," -f2- stripe_012.csv) <(cut -d"," -f2- stripe_013.csv) <(cut -d"," -f2- stripe_014.csv) <(cut -d"," -f2- stripe_015.csv)
