### A jófej és az írás

Probléma: "Parker és Brett pénzfeldobóst játszanak. Hány érme használata esetén lesz a fejek száma 25% valószínűséggel 3 darab? (Segítség:  lehet kísérletezéssel is kísérletezni, ez esetben mellékelje a mérési jegyzőkönyvet!)"

Legyen a használt pénzérmék száma n.

Így az összes lehetséges dobások száma: 2^n. (Egy pénzérmét ha feldobunk 2 fajta eredménye lehet)
Azoknak a dobásoknak a száma ahol pedig pontosan 3 fej van: $${n}\choose{3}$$. (Hány fajta képpen tudunk ebből az n darabból kiválasztani a 3 fejet amit várunk.)

Tehát ennek a valószínűsége megkapható úgy, hogy = (n!/(3!(n-3)!))/(2^n). (Kedvező esetek száma ösztva az összes eset számával)
A feladatnak csak egész megoldásai vannak ezért elég könnyen ábrázolni tudjuk ezt n függvényében: ![](https://i.imgur.com/1WTa99c.png)  
A grafikonról leolvashatjuk, hogy a megoldás a 4 lesz mivel **n=4** esetén a fenti kifejezés értéke 0.25 (Tehát a valószínűség 25%).
