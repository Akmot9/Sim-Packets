pub mod sim_packet;
use serde::Serialize;

/// État partagé de la simulation, émis vers le frontend via l'évènement
/// `system_state_update` pour piloter la barre de progression.
#[derive(Debug, Serialize, Clone, Default)]
pub struct SimPcapState {
    /// Nombre de paquets déjà envoyés depuis le début de la simulation.
    pub packet_sended: u32,
    /// Nombre total de paquets à envoyer (somme sur tous les fichiers).
    pub total_packets: u32,
    /// `true` tant que la simulation est en cours.
    pub sim_status: bool,
}
