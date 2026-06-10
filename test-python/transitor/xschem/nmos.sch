v {xschem version=3.4.6 file_version=1.2}
G {}
K {}
V {}
S {}
E {}
N 110 30 110 50 {lab=vs}
N 110 -150 110 -130 {lab=vdd}
N 400 70 400 90 {lab=vdd}
N 400 150 400 170 {lab=GND}
N 480 70 480 90 {lab=vref}
N 480 150 480 170 {lab=GND}
N 110 0 140 0 {lab=vs}
N 140 0 140 50 {lab=vs}
N 110 50 140 50 {lab=vs}
N 110 50 110 70 {lab=vs}
N 110 -50 110 -30 {lab=vout}
N 110 -50 130 -50 {lab=vout}
N 110 -70 110 -50 {lab=vout}
N 50 -0 70 0 {lab=vg}
N 110 130 110 150 {lab=GND}
C {sg13g2_pr/sg13_lv_nmos.sym} 90 0 0 0 {name=M2
l=6.4u
w=297u
ng=100
m=1
model=sg13_lv_nmos
spiceprefix=X
}
C {vsource.sym} 400 120 0 0 {name=V1 value=1.5 savecurrent=false}
C {lab_pin.sym} 400 70 2 0 {name=p1 sig_type=std_logic lab=vdd}
C {gnd.sym} 400 170 0 0 {name=l2 lab=GND}
C {lab_pin.sym} 110 -150 2 0 {name=p2 sig_type=std_logic lab=vdd}
C {vsource.sym} 480 120 0 0 {name=V3 value=1 savecurrent=false}
C {lab_pin.sym} 480 70 2 0 {name=p5 sig_type=std_logic lab=vref}
C {gnd.sym} 480 170 0 0 {name=l4 lab=GND}
C {lab_pin.sym} 130 -50 2 0 {name=p7 sig_type=std_logic lab=vout}
C {code_shown.sym} 300 -330 0 0 {name=NGSPICE only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

B1 vg 0 V = 0.9*(1+tanh(1000*(v(vref)-v(vout))))

.control
op
print vout vs vg

save @n.xm2.nsg13_lv_nmos[gm]

let gm = @n.xm2.nsg13_lv_nmos[gm]
print gm

.endc

"}
C {lab_pin.sym} 50 0 3 0 {name=p4 sig_type=std_logic lab=vg}
C {gnd.sym} 110 150 0 0 {name=l1 lab=GND}
C {res.sym} 110 -100 0 0 {name=R1
value=50k
footprint=1206
device=resistor
m=1}
C {isource.sym} 110 100 0 0 {name=I0 value=10e-6}
C {lab_pin.sym} 140 50 2 0 {name=p3 sig_type=std_logic lab=vs}
