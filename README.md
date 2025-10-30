# Wypełnianie siatki trójkątów - podstawowa specyfikacja:

Dana jest powierzchnia Beziera 3-go stopnia - wczytywana z pliku tekstowego przy starcie programu:
Format pliku:
16 linii (każda linia jeden punkt kontrolny), każda po 3 liczby rzeczywiste (x,y,z)
X00 Y00 Z00
X01 Y01 Z01
itd

Wartości punktów kontrolnych proszę tak dobrać aby:
- powierzchnia Beziera była funkcyjna względem płaszczyzny xy
- "bouding box" punktów kontrolnych był zbliżony do sześcianu oraz po dowolnym obrocie wokół jego środka "dał" się narysowac na "canvas'ie" aplikacji.
Prosze tak przyjąć układ współrzędnych "canvas'u" aby początek układu współrzędnych znajdował się w jego środku
Na przykład w Windows Forms uzyskujemy to poprzez:


Siatka trójkątów do wypełniania wyznaczana jest na podstawie triangulowanej (interpolacja trójkątami) powierzchni Beziera
Powierzchnia Beziera (czyli każdy wierzchołek 3D trójkąta) jest następnie obracana wokół osi z i osi x odpowiednio o kąty alfa i beta.
Po obrocie rysowany jest rzut prostokatny powierzchni na płaszczyznę xy (czyli wykorzystujemy tylko współrzędne x,y - współrzędna z pomijamy)
- Dokładność "triangulacji" - suwak
- Katy alfa (od -45 do 45 stopni) i beta (od 0 do 10 stopni) - suwaki
- Powinna istnieć mozliwość wybrania czy rysujemy tylko siatkę, czy tylko wypełnienie trójkątów

Można przyjąć następujące struktury danych:
- siatka: lista trójkatów
- trójkąt: 3 wierzchołki
- wierzchołek:
    punkt P przed obrotem, wektor styczny Pu przed obrotem, wektor styczny Pv przed obrotem, wektor normalny N przed obrotem
    punkt P po obrocie, wektor styczny Pu po obrocie, wektor styczny Pv po obrocie, wektor normalny N po obrocie
    2024-11-04: parametry u,v (na podstawie których wyliczyliśmy wierzchołek trójkata)
- punkt, wektor: 3 współrzędne x,y,z (mozna wykorzystać z .net namespace System.Numerics i klasę Vector3)

Przydatne wzory:

Obrót:

gdzie M3x3 odpowiednie macierze obrotu:

Wypełniamy niezależnie każdy zrzutowany trójkąt według poniższych zasad:

Algorytm wypełniania wielokątów/trójkątów:
 z sortowaniem krawędzi (kubełkowym) - (osoby o nazwiskach od A do K)
 z sortowaniem wierzchołków - (osoby o nazwiskach od L do Z)

Kolor wypełniania I:
Składowa rozproszona rmodelu oświetlenia (model Lamberta) + składowa zwierciadlana :
         I = kd*IL*IO*cos(kąt(N,L)) + ks*IL*IO*cosm(kąt(V,R))


(równanie oświetlenia raktujemy jako 3 niezalezne równania dla każdej składowej R,G,B koloru)
(cosinus kąta liczymy z iloczynu skalarnego wersorów N i L , np. cos(kąt(N,L)) = Nx*Lx+Ny*Ly+Nz*Lz )
 kd i ks - współczynniki opisujące wpływ danej składowej na wynik (0 - 1)
 IL(kolor światła) - możliwość wyboru z menu -> domyślnie kolor biały (1,1,1)
 IO(kolor obiektu)
 L (wersor do światła)
 N (Wektor normalny )
 V=[0,0,1], R=2<N,L>N-L gdzie <N,L> - iloczyn skalarny wersorów N i L
 Uwaga!! wszystkie wektory po wyznaczeniu muszą zostać znormalizowane do długości 1 (wersor)
 m - współczynnik opisujący jak bardzo dany trókat jest zwierciadlany (1-100)

Uwaga
  - jeśli cosinusy we wzorze wychodzą ujemne -> przyjmujemy wartości 0!
  - obliczenia wykonujemy dla wartości kolorów z przedziału 0..1, dopiero ostateczny wynik konwerujemy do przedziału 0..255 (obcinając do 255)

Kolor wypełnienia punktu wewnątrz trójkąta wyznaczany dokładnie w punkcie interpolując wektory normalne i współrzędną 'z 'do wnętrza' trójkąta
Uwaga
 Do interpolacji używamy współrzędnych barycentrycznych punktu wewnątrz trójkata

współczynniki kd, ks i m:
 podane jednakowe dla wszystkich trójkątów (suwaki)

Żródło światła - animacja (z opcja zatrzymania) ruchu po spirali na pewnej płaszczyźnie z=const (z - suwak)

IO(kolor obiektu) - radiobuttons:
 albo stały wybrany z menu
 wczytywana tekstura (obraz) całego 'panelu' -> domyślnie pewna tekstura. 2024-11-04: tekstura przeskalowana do wymiarów od 0 do 1, tak aby punkty tekstury odpowiadały parametrom u-v powierzchni

Powinna istnieć możliwość (checkbox) modyfikacji wektora normalnego na podstawie wczytanej mapy wektorów normalnych.
Zmodyfikowany wektor normalny: N = M*Ntekstury
  Ntekstury - wektor normalny(wersora) odczytany z koloru tekstury (NormalMap) dla całego 'panelu',
  Nx=<-1,+1>, Ny=<-1,+1>, Nz=<-1,+1> (składowa Nz powinna być dodatnia - dlatego Blue=128..255)
  M - macierz przekształcenia (obrotu) dla wektora z tekstury:
    M3x3 = [Pu, Pv, Npowierzchni]
  Npowierzchni - wektor normalny(wersor) odczytany/wyliczony z powierzchni
Możliwość zmiany (wczytania) domyślnej tekstury/mapy wektorów normalnych

Przykładowe mapy wektorów normalnych (NormalMap):

(np. RGB(127,127,255) => N=[0,0,1])
