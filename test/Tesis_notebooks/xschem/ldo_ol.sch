v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -40 -580 -40 -520 {
lab=vpos}
N -90 -580 -40 -580 {
lab=vpos}
N -180 -580 -150 -580 {
lab=vfb}
N -40 -460 -40 -440 {
lab=vss}
N 140 -860 140 -840 {
lab=vss}
N 140 -780 140 -760 {
lab=vout}
N 270 -860 270 -840 {
lab=vss}
N 140 -860 270 -860 {
lab=vss}
N 270 -780 270 -760 {
lab=vout}
N 140 -760 270 -760 {
lab=vout}
N 100 -860 140 -860 {
lab=vss}
N 20 -860 20 -790 {
lab=#net1}
N 140 -730 140 -710 {
lab=vout}
N 140 -640 140 -620 {
lab=vfb}
N 140 -550 140 -540 {
lab=vss}
N -350 -860 -230 -860 {
lab=#net1}
N -350 -790 -350 -780 {
lab=vss}
N -230 -790 -230 -780 {
lab=vss}
N -230 -860 -230 -850 {
lab=#net1}
N 100 -640 140 -640 {
lab=vfb}
N -350 -860 -350 -850 {
lab=#net1}
N 100 -860 100 -830 {
lab=vss}
N 140 -1020 240 -1020 {
lab=vss}
N -310 -840 -270 -840 {
lab=#net2}
N 20 -790 100 -790 {
lab=#net1}
N 360 -730 400 -730 {
lab=vout}
N 360 -730 360 -660 {
lab=vout}
N 260 -550 360 -550 {
lab=vss}
N 360 -600 360 -550 {
lab=vss}
N 140 -650 140 -640 {
lab=vfb}
N 140 -760 140 -730 {
lab=vout}
N 260 -730 360 -730 {
lab=vout}
N 140 -560 140 -550 {
lab=vss}
N 140 -1020 140 -860 {
lab=vss}
N -350 -780 -230 -780 {
lab=vss}
N 90 -860 100 -860 {
lab=vss}
N 20 -860 30 -860 {
lab=#net1}
N 90 -760 140 -760 {
lab=vout}
N 20 -760 30 -760 {
lab=#net1}
N 20 -790 20 -760 {
lab=#net1}
N -160 -860 -160 -850 {
lab=#net1}
N -160 -860 20 -860 {
lab=#net1}
N -230 -860 -160 -860 {
lab=#net1}
N -230 -780 -160 -780 {
lab=vss}
N -160 -790 -160 -780 {
lab=vss}
N 260 -730 260 -660 {
lab=vout}
N 140 -730 260 -730 {
lab=vout}
N 260 -600 260 -550 {
lab=vss}
N 140 -550 260 -550 {
lab=vss}
N -270 -840 -270 -710 {
lab=#net2}
N -310 -690 -310 -670 {
lab=vpos}
N -310 -800 -310 -750 {
lab=vol}
C {res.sym} -120 -580 1 0 {name=R3
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} -40 -490 0 0 {name=C1
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -180 -580 0 0 {name=p13 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -40 -580 2 0 {name=p8 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -40 -440 3 0 {name=p14 sig_type=std_logic lab=vss}
C {devices/res.sym} 270 -810 0 0 {name=Ro_pt
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 140 -810 2 1 {name=Gm_pt value=1}
C {devices/res.sym} 140 -680 0 0 {name=R1
value=1
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 140 -590 0 0 {name=R2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -350 -820 0 1 {name=Gma value=1}
C {devices/res.sym} -230 -820 2 0 {name=Ra
value=1
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 100 -640 0 0 {name=p10 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 240 -1020 3 0 {name=p11 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -350 -780 0 0 {name=p12 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 140 -540 3 0 {name=p15 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 400 -730 2 0 {name=p16 sig_type=std_logic lab=vout}
C {devices/res.sym} 360 -630 0 0 {name=Rl
value=1
footprint=1206
device=resistor
m=1}
C {capa.sym} 60 -860 3 0 {name=Cgg_pt
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 60 -760 3 0 {name=Cgd_pt
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -160 -820 0 0 {name=Ca
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 260 -630 0 0 {name=Cl
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -270 -710 3 0 {name=p18 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -310 -720 0 0 {name=V3 value=3 savecurrent=false}
C {devices/lab_pin.sym} -310 -670 3 0 {name=p19 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -310 -760 0 0 {name=p20 sig_type=std_logic lab=vol}
