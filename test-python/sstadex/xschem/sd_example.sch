v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 80 -30 130 -30 {
lab=vss}
N 130 -30 130 30 {
lab=vss}
N 80 30 130 30 {
lab=vss}
N 80 0 80 30 {
lab=vss}
N 80 -100 80 -60 {
lab=vout}
N 230 -100 230 -40 {
lab=vout}
N 80 120 230 120 {
lab=vss}
N 230 20 230 120 {
lab=vss}
N 80 -140 80 -100 {
lab=vout}
N 80 -220 80 -200 {
lab=vdd}
N 80 -100 230 -100 {
lab=vout}
N 80 90 80 120 {
lab=vss}
N -240 30 -240 90 {
lab=vss}
N -240 90 80 90 {
lab=vss}
N 80 30 80 90 {
lab=vss}
N 400 -100 400 -40 {
lab=vfb}
N -90 -30 40 -30 {
lab=vg1}
N -240 -30 -150 -30 {
lab=#net1}
N 400 20 400 120 {
lab=vss}
N 230 120 400 120 {
lab=vss}
N 230 -100 290 -100 {
lab=vout}
N 350 -100 400 -100 {
lab=vfb}
C {sg13g2_pr/sg13_hv_nmos.sym} 60 -30 2 1 {name=M1
l=1.6u
w=5.1u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 230 -100 1 0 {name=p1 sig_type=std_logic lab=vout}
C {capa.sym} 230 -10 0 0 {name=C1
m=1
value=1p
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -1080 -340 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt
.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

Vdd vdd 0 3.3
Vss vss 0 0
Vref vref 0 1.2

.param vcc=3.3

.control
ac dec 1000 1 1G
plot vdb(vout)
wrdata sd_example1.csv vdb(vout)
op 
print vout
print vfb
print vg1
print vg2
let gm_diff = @n.xm1.nsg13_hv_nmos[gm]
let id_diff = @n.xm1.nsg13_hv_nmos[ids]
let gds_diff = @n.xm1.nsg13_hv_nmos[gds]
let cgg_diff = @n.xm1.nsg13_hv_nmos[cgg]
print gm_diff
print gds_diff
print id_diff
print cgg_diff
print gm_diff/id_diff
.endc

"}
C {devices/lab_pin.sym} 80 120 3 0 {name=p2 sig_type=std_logic lab=vss}
C {isource.sym} 80 -170 0 0 {name=I1 value=14.7e-6}
C {devices/lab_pin.sym} 80 -220 1 0 {name=p4 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 0 -30 3 0 {name=p3 sig_type=std_logic lab=vg1}
C {bsource.sym} -240 0 0 0 {name=B1 VAR=V FUNC="vcc/2*(1+tanh(10000*(v(vfb)-v(vref))))"}
C {vsource.sym} -120 -30 1 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {res.sym} 320 -100 1 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 400 -10 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 400 -100 2 0 {name=p5 sig_type=std_logic lab=vfb}
