# Example 9.2 test outline:

Rainfall intensity:
Time (min) 5 10 15 20 30 40 50 60 120
Intensity (in/h) 7.1 5.9 5.1 4.5 3.5 3.0 2.6 2.4 1.4

summary of structures:

Structure Number, Structure Type, Drainage Area(ac), Runoff Coefficient, Time of Concentration(min)
40 Inlet 0.64 0.73 3
41 Inlet 0.35 0.73 2
42 Inlet 0.32 0.73 2
43 Access hole n/a n/a n/a
44 Outlet n/a n/a n/a

Rim elevations:
40 - 370.0
41 - 360.0
42 - 349.31
43 - 347.76
44 - n/a (outfall)

Pipe 40-41: 18in pipe at 0.03 slope. Upstream invert 365.5. Downstream invert 354.67. Length is 361ft

Pipe 41-42: 18in pipe, at 0.03 slope. Upstream invert 354.07. Downstream 344.23 (length 344.23 ft)

Pipe 42-43: 24in pipe, at 0.001 slope. Upstream invert 344.07 Downstream 344.06 length 14.1ft

Pipe 43-44: 24in pipe, at 0.01 slope. Upstream invert 331.27. Downstream 330.71. Length 55.8ft

The tailwater HGL = EGL = 333.5

Solve HGL and EGL

assert 43-44 downstream HGL = 335.50 and EGL = 333.57

assert 43-44 upsteam HGL = 333.55 and EGL = 333.62

assert 43 HGL = 333.68 and it is not flooded

assert 42-43 downstream HGL = 345.62 and EGL = 345.72

assert 42-43 upstream HGL = 345.63 and EGL = 345.73

assert 42 HGL = 345.82 and EGL = 345.81 and it is not flooded

assert 41-42 downstream HGL = 345.73 and EGL = 345.86

assert 41-42 upstream HGL = 354.63 and EGL = 355.85

assert 41 EGL = 355.85 and is not flooded.

assert 40-41 downstream HGL = 355.80 and EGL = 355.88

assert 40-41 upstream EGL = 366.85