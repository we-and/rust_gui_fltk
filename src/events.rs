use fltk::dialog;
use fltk::{
    app,
    button::*,
    group::{Flex, Tabs},
    input::Input,
    menu::{Choice, MenuBar, MenuButton},
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};
use fltk::{enums::*, frame::Frame, group::*, menu::*, prelude::*, window::*};
use fltk::{prelude::*, *};
//mod constants;
use crate::constants::constants::{WIN_WIDTH, WIN_HEIGHT, MENU_HEIGHT,LEFT_WIDTH,RIGHT_WIDTH,LEFT_TOP_HEIGHT,PANEL_PADDING,LOGO_WIDTH};


#[derive(Debug, Copy, Clone)]
pub enum Message {
    ComboBox_LanguageChanged,
    Button_NuovaSessione,
    Button_CancellaSessione,
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



pub fn handle_menu_messages(msg:Message     ){

    match msg {
        Message::ComboBox_LanguageChanged =>{
            println!("Language Changed");
          
        },
        Message::Button_NuovaSessione =>{
            println!("Nuova sessione");
            
        }, Message::Button_CancellaSessione =>{
            println!("Cancella sessione");
            
        },
        Message::File_CorpusNuovo=> { // Call your function here
            println!("File > Corpus Nuovo");
        },
        Message::File_CorpusEsistente=> { },
        Message::File_ScanParseAlfabeto=> { },
        Message::File_ScanParseSeparator=> { },
        Message::File_Importa_MieRisorse=> { },
        Message::File_Importa_TabellaInDataset=> { },
        Message::File_Importa_ListaSempliceInDataset=> { },
        Message::File_Esporta_DocumentiTipi=> { },
        Message::File_Esporta_TipiVariabili=> { },
        Message::File_Esporta_RiconstruzioneCorpus=> { },
        Message::File_Esporta_Subcorpus=> { },
        Message::File_Esci=> { },
        Message::Analisi_Pretratamento=> { },
        Message::Analisi_Lessicale_Tagging=> { },
        Message::Analisi_Lessicale_EstrazioneDiParoleChiave=> { },
        Message::Analisi_Testuale=> { },
        Message::Impostationi_ModalitaCore_MonoCore=> { },
        Message::Impostationi_ModalitaCore_MultiCore=> { },
        Message::Impostationi_ModalitaSchermo=> { },
        Message::Info_SuTalTac4=> { },
        Message::Info_Licenza=> { },
        Message::Info_ManualeUtente=> { },
        Message::Info_Tutorial=> { },
        Message::Info_Supporto=> {}
    }
}
