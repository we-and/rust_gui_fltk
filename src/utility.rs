use fltk::dialog;
#[derive(Debug, Copy, Clone)]
pub enum Message {
    File_CorpusNuovo,
    File_CorpusEsistente,
    File_ScanParseAlfabeto,
    File_ScanParseSeparator,
    File_Importa_MieRisorse,
    File_Importa_TabellaInDataset,
    File_Importa_ListaSempliceInDataset,
    File_Esporta_DocumentiTipi,
    File_Esporta_TipiVariabili,
    File_Esporta_RiconstruzioneCorpus,
    File_Esporta_Subcorpus,
    File_Esci,
    Analisi_Pretratamento,
    Analisi_Lessicale_Tagging,
   Analisi_Lessicale_EstrazioneDiParoleChiave,
    Analisi_Testuale,
    Impostationi_ModalitaCore_MonoCore,
    Impostationi_ModalitaCore_MultiCore,
    Impostationi_ModalitaSchermo,
    Info_SuTalTac4,
    Info_Licenza,
    Info_ManualeUtente,
    Info_Tutorial,
    Info_Supporto
}

// credit: https://stackoverflow.com/questions/30811107/how-do-i-get-the-first-character-out-of-a-string#comment83958722_48482196
pub fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}
