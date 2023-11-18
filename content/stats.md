+++
title = "Statistics"
[extra]
primary_navigation = true
+++
## Population
{{ chart(source="population", caption="Population per year", label="Population (Billions)", factor=1000000000) }}

{{ chart(source="population", caption="Life expectancy at birth per year", label="Life expectancy at birth (Years)", column=2) }}

{{ chart(source="population", caption="Infant mortality per year", label="Infant mortality rate (per 1,000 births)", column=3) }}

{{ chart(source="population", caption="Median age per year", label="Median age (Years)", column=4) }}

All population data is taken from The 2022 Revision of World Population Prospects, which is the <q cite="https://population.un.org/wpp/">twenty-seventh edition of official United Nations population estimates and projections that have been prepared by the Population Division of the Department of Economic and Social Affairs of the United Nations Secretariat. It presents population estimates from 1950 to the present for 237 countries or areas, underpinned by analyses of historical demographic trends. This latest assessment considers the results of 1,758 national population censuses conducted between 1950 and 2022, as well as information from vital registration systems and from 2,890 nationally representative sample surveys The 2022 revision also presents population projections to the year 2100 that reflect a range of plausible outcomes at the global, regional and national levels.</q>

## Atmosphere
{{ chart(source="atmosphere", caption="Atmospheric Carbon per year", label="Atmospheric CO2 (PPM)") }}

All atmospheric carbon data comes from the Mauna Loa Observatory, Hawaii. <q cite="https://gml.noaa.gov/ccgg/trends/">The carbon dioxide data on Mauna Loa constitute the longest record of direct measurements of CO2 in the atmosphere. They were started by C. David Keeling of the Scripps Institution of Oceanography in March of 1958 at a facility of the National Oceanic and Atmospheric Administration [Keeling, 1976]. NOAA started its own CO2 measurements in May of 1974, and they have run in parallel with those made by Scripps since then [Thoning, 1989].</q>

{{ chart(source="temperature", caption="Global Temperature Anomaly per year", label="Global Temperature Anomaly (°C)", negative=true) }}

Data about past temperatures comes from the NOAA National Centers for Environmental information, Climate at a Glance which <q cite="https://www.ncei.noaa.gov/access/monitoring/climate-at-a-glance/">provides global-scale temperature information using data from NOAA's Global Surface Temperature Analysis (NOAAGlobalTemp), which uses comprehensive data collections of increased global coverage over land (Global Historical Climatology Network-Monthly) and ocean (Extended Reconstructed Sea Surface Temperature) surfaces. The tool provides near real-time analysis of monthly and annual temperatures for the globe and is intended for the study of climate variability and change. Data is provided globally, by hemisphere, and by land and ocean surface components. The interactive mapping tool allows analysis of the spatial patterns of global temperature anomalies and regional temperature analysis. Recent data are preliminary and may be modified after appropriate quality control has been performed.</q>

{{ chart(source="ozone", caption="Mean ozone hole size for 07 September to 13 October per year", label="Mean Ozone Hole Size (million ㎢)") }}

Data on the global ozone hole size comes from NASA Ozone Watch. The global ozone hole <q cite="https://ozonewatch.gsfc.nasa.gov/facts/hole_SH.html">is not technically a “hole” where no ozone is present, but is actually a region of exceptionally depleted ozone in the stratosphere over the Antarctic that happens at the beginning of Southern Hemisphere spring (August–October).</q>.

## Biodiversity
{{ chart(source="living_planet", caption="Living Planet Index per year.", label="Living Planet Index") }}

The Living Planet Index (LPI) <q cite="https://www.livingplanetindex.org/">is a measure of the state of the world's biological diversity based on population trends of vertebrate species from terrestrial, freshwater and marine habitats. The LPI was adopted by the Convention of Biological Diversity (CBD) as an indicator of progress towards its 2011-2020 targets and can play an important role in monitoring progress towards the post-2020 goals and targets negotiated at COP15.</q>

{{ chart(source="red_list_index", caption="IUCN Red List Index of species survival per year.", label="Red List Index") }}

The IUCN Red List Index is derived from the Red List of Threatened Species and shows <q cite="https://www.iucnredlist.org/assessment/red-list-index">trends in the status of groups of species based only on genuine improvements or deteriorations in status of sufficient magnitude to qualify species for listing in more threatened or less threatened Red List Categories</q>. A value of 1.0 here equates to all species qualifying as Least Concern (i.e., not expected to become Extinct in the near future), whereas a value of 0 equates to all species having gone Extinct.