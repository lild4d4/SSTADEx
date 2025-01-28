v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -940 -60 -890 -60 {
lab=#net1}
N -890 -60 -860 -60 {
lab=#net1}
N -860 -60 -830 -60 {
lab=#net1}
N -940 10 -940 20 {
lab=vss}
N -940 20 -940 40 {
lab=vss}
N -940 20 -820 20 {
lab=vss}
N -820 10 -820 20 {
lab=vss}
N -820 -60 -820 -50 {
lab=#net1}
N -940 -60 -940 -50 {
lab=#net1}
N -860 -40 -860 0 {
lab=vn}
N -900 -40 -860 -40 {
lab=vn}
N -860 0 -860 40 {
lab=vn}
N -640 -130 -640 -110 {
lab=vdd}
N -640 -50 -640 -30 {
lab=vout}
N -510 -130 -510 -120 {
lab=vdd}
N -640 -130 -510 -130 {
lab=vdd}
N -510 -120 -510 -110 {
lab=vdd}
N -510 -50 -510 -30 {
lab=vout}
N -640 -30 -510 -30 {
lab=vout}
N -670 -130 -640 -130 {
lab=vdd}
N -680 -130 -670 -130 {
lab=vdd}
N -640 -30 -640 20 {
lab=vout}
N -640 -260 -640 -130 {
lab=vdd}
N -680 -130 -680 -100 {
lab=vdd}
N -830 -60 -820 -60 {
lab=#net1}
N -780 -70 -780 -60 {
lab=#net1}
N -780 -130 -680 -130 {
lab=vdd}
N -690 -30 -640 -30 {
lab=vout}
N -750 -60 -750 -30 {
lab=#net1}
N -820 -60 -750 -60 {
lab=#net1}
N -750 -60 -680 -60 {
lab=#net1}
N -640 0 -560 0 {
lab=vout}
N -640 80 -640 110 {
lab=vss}
N -510 -130 -450 -130 {
lab=vdd}
N -450 -130 -440 -130 {
lab=vdd}
N -440 -130 -440 -110 {
lab=vdd}
N -510 -30 -440 -30 {
lab=vout}
N -440 -50 -440 -30 {
lab=vout}
C {devices/vccs.sym} -940 -20 2 0 {name=Gma value=0.0004627}
C {devices/res.sym} -820 -20 2 0 {name=Ra
value=55104
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -900 0 2 0 {name=p3 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} -940 40 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} -860 40 2 0 {name=p9 sig_type=std_logic lab=vn}
C {devices/res.sym} -510 -80 0 0 {name=Ro_pt
value=8643
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -640 -80 2 1 {name=Gm_pt value=2.7615e-5}
C {capa.sym} -720 -30 1 0 {name=C1
m=1
value=1.87e-13
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -780 -100 2 0 {name=C2
m=1
value=3.07e-13
footprint=1206
device="ceramic capacitor"}
C {devices/isource.sym} -640 50 0 0 {name=I1 value=0}
C {devices/lab_pin.sym} -560 0 2 0 {name=p6 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -640 110 3 0 {name=p10 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} -640 -260 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/code_shown.sym} -940 190 0 0 {name=s2 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0
Vdd vdd 0 0
Vpos vp 0 0 ac 1
Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)


.endc

"}
C {capa.sym} -440 -80 0 0 {name=C3
m=1
value=5.7e-14
footprint=1206
device="ceramic capacitor"}
