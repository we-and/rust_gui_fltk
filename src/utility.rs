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

