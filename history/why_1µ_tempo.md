La tempo de 1 microseconde (µs) a été ajoutée dans le code pour réguler la fréquence d'envoi des paquets. Voici pourquoi elle est importante :

### Raison de la tempo de 1 µs :
- **Limiter la vitesse d'envoi** : Sans tempo, les paquets sont envoyés aussi vite que le code peut les traiter, ce qui peut dépasser la capacité de réception de la carte réseau ou du programme de capture. Cela peut entraîner la perte de paquets ou une surcharge du système.
- **Assurer la réception complète des paquets** : En ajoutant un délai très court (1 µs), on s'assure que le code de réception et la carte réseau ont juste assez de temps pour traiter chaque paquet avant que le suivant ne soit envoyé. Cela évite de saturer la file d'attente et réduit les risques de perte de paquets.
- **Tests de performance** : La tempo de 1 µs a permis de tester la limite de la capacité de réception et de s'assurer que la performance reste stable à une fréquence d'envoi élevée.

### Résultat de l'ajout de la tempo :
Avec cette tempo de 1 µs, le système a été capable de réceptionner tous les paquets sans perte, montrant ainsi qu'il peut gérer des envois très rapides sans compromettre la réception. Cela a confirmé que le problème initial de perte de paquets était dû à une fréquence d'envoi trop élevée sans régulation.