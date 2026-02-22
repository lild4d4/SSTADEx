.PHONY: all
all: lut_gen lut_show

.PHONY: lut_gen
lut_gen: ../../LUTs/ihp-sg13g2/lv_10w.npz

../../LUTs/ihp-sg13g2/lv_10w.npz: lut_gen.py
	python lut_gen.py

.PHONY: lut_show
lut_show:
	mkdir -p ../../LUTs/ihp-sg13g2
	python lut_show.py
