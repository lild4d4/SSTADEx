v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 590 -160 590 -140 {
lab=vout}
N 590 -70 590 -50 {
lab=vfb}
N 590 20 590 30 {
lab=vss}
N 550 -70 590 -70 {
lab=vfb}
N 810 -160 810 -90 {
lab=vout}
N 710 -160 710 -90 {
lab=vout}
N 710 -30 710 20 {
lab=vss}
N 590 20 710 20 {
lab=vss}
N 710 20 810 20 {
lab=vss}
N 810 -30 810 20 {
lab=vss}
N 810 -160 890 -160 {
lab=vout}
N 590 -220 630 -220 {
lab=vdd}
N 630 -270 630 -220 {
lab=vdd}
N 590 -270 630 -270 {
lab=vdd}
N 590 -270 590 -250 {
lab=vdd}
N 590 -80 590 -70 {
lab=vfb}
N 710 -160 810 -160 {
lab=vout}
N 590 -160 710 -160 {
lab=vout}
N 590 10 590 20 {
lab=vss}
N 590 -190 590 -160 {
lab=vout}
N 300 -220 550 -220 {
lab=vmid}
N 140 60 300 60 {
lab=vs}
N 120 60 120 110 {
lab=vs}
N -190 -60 -100 -60 {
lab=#net1}
N -60 60 120 60 {
lab=vs}
N -60 -480 300 -480 {
lab=vdd}
N -60 -480 -60 -450 {
lab=vdd}
N 300 -340 300 -320 {
lab=vcp_cm}
N 300 -480 300 -450 {
lab=vdd}
N 300 -100 300 -90 {
lab=vcp}
N 300 -30 300 60 {
lab=vs}
N -60 -30 -60 60 {
lab=vs}
N 340 -60 360 -60 {
lab=vn}
N 140 -60 300 -60 {
lab=vs}
N 140 -60 140 60 {
lab=vs}
N 120 -420 260 -420 {
lab=#net2}
N 120 -420 120 -220 {
lab=#net2}
N -110 -420 -60 -420 {
lab=vdd}
N -110 -480 -110 -420 {
lab=vdd}
N -110 -480 -60 -480 {
lab=vdd}
N 300 -420 350 -420 {
lab=vdd}
N 350 -480 350 -420 {
lab=vdd}
N 300 -480 350 -480 {
lab=vdd}
N -60 -60 140 -60 {
lab=vs}
N 120 60 140 60 {
lab=vs}
N -20 -420 120 -420 {
lab=#net2}
N -60 -100 -60 -90 {
lab=#net3}
N -20 -150 260 -150 {
lab=vbias_dp}
N 300 -220 300 -180 {
lab=vmid}
N -60 -220 -60 -180 {
lab=#net2}
N -100 -150 -60 -150 {
lab=#net3}
N -100 -150 -100 -100 {
lab=#net3}
N -100 -100 -60 -100 {
lab=#net3}
N -60 -120 -60 -100 {
lab=#net3}
N 300 -150 340 -150 {
lab=vcp}
N 340 -150 340 -100 {
lab=vcp}
N 300 -100 340 -100 {
lab=vcp}
N 300 -120 300 -100 {
lab=vcp}
N 120 170 120 260 {
lab=vss}
N 360 -60 360 -30 {
lab=vn}
N -20 -290 260 -290 {
lab=vbias_cm}
N 300 -260 300 -220 {
lab=vmid}
N -60 -220 120 -220 {
lab=#net2}
N -60 -260 -60 -220 {
lab=#net2}
N -60 -340 -60 -320 {
lab=#net4}
N -90 -290 -60 -290 {
lab=#net4}
N -90 -340 -90 -290 {
lab=#net4}
N -90 -340 -60 -340 {
lab=#net4}
N -60 -390 -60 -340 {
lab=#net4}
N 300 -290 330 -290 {
lab=vcp_cm}
N 330 -340 330 -290 {
lab=vcp_cm}
N 300 -340 330 -340 {
lab=vcp_cm}
N 300 -390 300 -340 {
lab=vcp_cm}
N 590 -480 590 -270 {
lab=vdd}
N 350 -480 590 -480 {
lab=vdd}
N 500 140 500 200 {
lab=vpos}
N 450 140 500 140 {
lab=vpos}
N 360 140 390 140 {
lab=vfb}
N 500 260 500 280 {
lab=vss}
N -190 -60 -190 50 {
lab=#net1}
N -190 100 -190 130 {
lab=vpos}
C {devices/lab_pin.sym} 890 -160 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/res.sym} 590 -110 0 0 {name=R1
value=5000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 590 -20 0 0 {name=R2
value=15000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 550 -70 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 590 30 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 710 -60 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 490 -220 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/code_shown.sym} -880 -460 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 
Vss vss 0 0
Vbias_dp vbias_dp 0 2.2
Vbias_cm vbias_cm vdd -0.88

.control
ac dec 10 1 10G
plot vdb(vout) 
plot phase(vout)*180/PI
op
print vout vmid vcp vfb vs

let gm_m1 = @n.xm1.nsg13_hv_nmos[gm]
let gm_m3 = @n.xm3.nsg13_hv_nmos[gm]
let gm_m6 = @n.xm6.nsg13_hv_pmos[gm]
let gm_m8 = @n.xm8.nsg13_hv_pmos[gm]
let gm_m9 = @n.xm9.nsg13_hv_pmos[gm]

let gds_m1 = @n.xm1.nsg13_hv_nmos[gds]
let gds_m3 = @n.xm3.nsg13_hv_nmos[gds]
let gds_m6 = @n.xm6.nsg13_hv_pmos[gds]
let gds_m8 = @n.xm8.nsg13_hv_pmos[gds]

print gm_m1
print gm_m3
print gm_m6
print gm_m8
print gm_m9

print 1/gds_m1
print 1/gds_m3
print 1/gds_m6
print 1/gds_m8

let ro_dp = gm_m3/(gds_m3*gds_m1)
let ro_cm = gm_m8/(gds_m8*gds_m6)
print ro_dp
print ro_cm

print ro_dp*ro_cm/(ro_dp+ro_cm)
.endc

"}
C {sg13g2_pr/sg13_hv_pmos.sym} -40 -290 0 1 {name=M7
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/isource.sym} 120 140 0 0 {name=I2 value=20e-6}
C {devices/lab_pin.sym} 120 60 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 100 -480 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 360 -30 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 120 260 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 320 -60 2 0 {name=M1
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -80 -60 2 1 {name=M2
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 280 -150 2 1 {name=M3
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -40 -150 2 0 {name=M4
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} -40 -420 0 1 {name=M5
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 280 -420 0 0 {name=M6
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 110 -150 3 0 {name=p10 sig_type=std_logic lab=vbias_dp}
C {devices/lab_pin.sym} 340 -100 2 0 {name=p11 sig_type=std_logic lab=vcp}
C {sg13g2_pr/sg13_hv_pmos.sym} 280 -290 0 0 {name=M8
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 570 -220 0 0 {name=M9
l=0.4u
w=2489u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 180 -290 3 0 {name=p13 sig_type=std_logic lab=vbias_cm}
C {devices/lab_pin.sym} 300 -370 2 0 {name=p14 sig_type=std_logic lab=vcp_cm}
C {res.sym} 420 140 1 0 {name=R4
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 500 230 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 360 140 0 0 {name=p3 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 500 140 2 0 {name=p15 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 500 280 3 0 {name=p16 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -190 130 2 0 {name=p12 sig_type=std_logic lab=vpos}
C {vsource.sym} -190 70 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {devices/res.sym} 810 -60 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
