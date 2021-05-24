#   Oxygen setup.
read -rsp $'Setup: oxygen diffusion\nPress any key to continue...' -n1 key;
cartographer output/inv/cartographer/skin input/ inv/cartographer/skin.json5;
babbage output/tmp input/ inv/babbage/build_map_diff_oxy.json5;
mv output/tmp/diff_oxy.nc input/res/maps/;
babbage output/tmp input/ inv/babbage/build_map_source_oxy.json5;
mv output/tmp/source_oxy.nc input/res/maps/;
diffuse output/inv/diffuse/oxy input/ inv/diffuse/oxy.json5;
mv output/inv/diffuse/oxy/099_diff.nc input/res/maps/init_oxy.nc;


# cp output/source_oxy.nc input/res/maps/;
# diffuse input/ output/diffuse/oxy diffuse/oxy.json5;
# cp output/diffuse/oxy/008_diff.nc input/res/maps/init_oxy.nc;
# touch done_oxy.txt;

# #   ALA setup.
# cartographer input output/cartographer/cream cartographer/cream.json5;
# babbage input/ output/ babbage/build_map_init_ala.json5;
# cp output/init_ala.nc input/res/maps/;
# babbage input/ output/ babbage/build_map_diff_ala.json5;
# cp output/diff_ala.nc input/res/maps/;
# touch done_ala.txt


# #   Shallow tumour
# #   PpIX setup.
# cartographer input output/cartographer/tumour_shallow cartographer/tumour_shallow.json5;
# babbage input/ output/ babbage/build_map_multipliers_tumour_shallow.json5;
# cp output/multipliers_shallow.nc input/res/maps/;
# cp output/cartographer/tumour_shallow/map_\{tumour\}.nc input/res/maps/tumour_shallow.nc;
# reactor input/ output/reactor/ppix/shallow reactor/ppix_shallow.json5;
# cp output/reactor/ppix/shallow/008_\{ppix\}_diff.nc input/res/maps/init_ppix_shallow.nc
# cp output/reactor/ppix/shallow/008_\{ala\}_diff.nc input/res/maps/init_ala_shallow.nc

# #   PDT phase.
# mcrt input/ output/mcrt/shallow mcrt/tumour_shallow.json5;
# cp output/mcrt/shallow/shift_density.nc input/res/maps/udens_shallow.nc;
# reactor input/ output/reactor/pdt/shallow reactor/pdt_shallow.json5;
# babbage input/ output/ babbage/build_map_tumour_shallow_kill.json5;
# touch done_tumour_shallow.txt


# #   Thick tumour
# #   PpIX setup.
# cartographer input output/cartographer/tumour_thick cartographer/tumour_thick.json5;
# babbage input/ output/ babbage/build_map_multipliers_tumour_thick.json5;
# cp output/multipliers_thick.nc input/res/maps/;
# cp output/cartographer/tumour_thick/map_\{tumour\}.nc input/res/maps/tumour_thick.nc;
# reactor input/ output/reactor/ppix/thick reactor/ppix_thick.json5;
# cp output/reactor/ppix/thick/008_\{ppix\}_diff.nc input/res/maps/init_ppix_thick.nc
# cp output/reactor/ppix/thick/008_\{ala\}_diff.nc input/res/maps/init_ala_thick.nc

# #   PDT phase.
# mcrt input/ output/mcrt/thick mcrt/tumour_thick.json5;
# cp output/mcrt/thick/shift_density.nc input/res/maps/udens_thick.nc;
# reactor input/ output/reactor/pdt/thick reactor/pdt_thick.json5;
# babbage input/ output/ babbage/build_map_tumour_thick_kill.json5;
# touch done_tumour_thick.txt


# #   Deep tumour
# #   PpIX setup.
# cartographer input output/cartographer/tumour_deep cartographer/tumour_deep.json5;
# babbage input/ output/ babbage/build_map_multipliers_tumour_deep.json5;
# cp output/multipliers_deep.nc input/res/maps/;
# cp output/cartographer/tumour_deep/map_\{tumour\}.nc input/res/maps/tumour_deep.nc;
# reactor input/ output/reactor/ppix/deep reactor/ppix_deep.json5;
# cp output/reactor/ppix/deep/008_\{ppix\}_diff.nc input/res/maps/init_ppix_deep.nc
# cp output/reactor/ppix/deep/008_\{ala\}_diff.nc input/res/maps/init_ala_deep.nc

# #   PDT phase.
# mcrt input/ output/mcrt/deep mcrt/tumour_deep.json5;
# cp output/mcrt/deep/shift_density.nc input/res/maps/udens_deep.nc;
# reactor input/ output/reactor/pdt/deep reactor/pdt_deep.json5;
# babbage input/ output/ babbage/build_map_tumour_deep_kill.json5;
# touch done_tumour_deep.txt


#   Shaped tumour
#   PpIX setup.
cartographer input output/cartographer/tumour_shaped cartographer/tumour_shaped.json5;
babbage input/ output/ babbage/build_map_multipliers_tumour_shaped.json5;
cp output/multipliers_shaped.nc input/res/maps/;
cp output/cartographer/tumour_shaped/map_\{tumour\}.nc input/res/maps/tumour_shaped.nc;
reactor input/ output/reactor/ppix/shaped reactor/ppix_shaped.json5;
cp output/reactor/ppix/shaped/008_\{ppix\}_diff.nc input/res/maps/init_ppix_shaped.nc
cp output/reactor/ppix/shaped/008_\{ala\}_diff.nc input/res/maps/init_ala_shaped.nc

#   PDT phase.
mcrt input/ output/mcrt/shaped mcrt/tumour_shaped.json5;
cp output/mcrt/shaped/shift_density.nc input/res/maps/udens_shaped.nc;
reactor input/ output/reactor/pdt/shaped reactor/pdt_shaped.json5;
babbage input/ output/ babbage/build_map_tumour_shaped_kill.json5;
touch done_tumour_shaped.txt
