
def solv(ii):
    maxt = 20
    for good in range(0, maxt+1):
        bad = maxt-good
        pont = round((good-bad)/5.0)
        if pont == ii:
            print(good)

for ii in range(0,5):
    print(ii)
    print("===")
    solv(ii)
    print("====")
