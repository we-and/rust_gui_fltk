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

const LOGO_WIDTH:i32=180;

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


pub fn build_menu(s: app::Sender<Message>,WIN_WIDTH :i32) {
  

    let mut menu_bar = MenuBar::new(LOGO_WIDTH, 0, WIN_WIDTH, 30, None);
    menu_bar.add_emit(
        "&File/Corpus Nuovo\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_CorpusNuovo,
    );
    menu_bar.add_emit(
        "&File/Corpus esistente\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_CorpusEsistente,
    );    menu_bar.add_emit(
        "&File/Scan + Parse Alfabeto\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_ScanParseAlfabeto,
    );    menu_bar.add_emit(
        "&File/Scan + Parse Separatori\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_ScanParseSeparator,
    );

    menu_bar.add_emit(
        "&File/Importa/Mie risorse\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Importa_MieRisorse,
    );
    menu_bar.add_emit(
        "&File/Importa/Tabella in Dataset\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Importa_TabellaInDataset,
    );

    menu_bar.add_emit(
        "&File/Importa/Lista semplice in Dataset\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Importa_ListaSempliceInDataset,
    );
    menu_bar.add_emit(
        "&File/Esporta/Documenti - tipi",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Esporta_DocumentiTipi,
    );

    menu_bar.add_emit(
        "&File/Esporta/Tipi - Variabili",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Esporta_TipiVariabili,
    );

    menu_bar.add_emit(
        "&File/Esporta/Riconstruzione Corpus",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Esporta_RiconstruzioneCorpus,
    );

    menu_bar.add_emit(
        "&File/Esporta/Subcorpus",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Esporta_Subcorpus,
    );

    menu_bar.add_emit(
        "&File/Esci\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File_Esci,
    );
    menu_bar.add_emit(
        "Analisi/Pre-Tratamento\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Analisi_Pretratamento,
    );
    menu_bar.add_emit(
        "Analisi/Lessicale/Tagging\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Analisi_Lessicale_Tagging,
    );
    menu_bar.add_emit(
        "Analisi/Lessicale/Estrazione di Parole Chiave\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Analisi_Lessicale_EstrazioneDiParoleChiave,
    );
    menu_bar.add_emit(
        "Analisi/Testuale\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Analisi_Testuale,
    );

    menu_bar.add_emit(
        "Impostationi/Modalità Core/Mono-Core\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Impostationi_ModalitaCore_MonoCore,
    );
    menu_bar.add_emit(
        "Impostationi/Modalità Core/Multi-Core\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Impostationi_ModalitaCore_MultiCore,
    );
    menu_bar.add_emit(
        "Impostationi/Modalità Schermo\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Impostationi_ModalitaSchermo,
    );
    menu_bar.add_emit(
        "Info/Su TalTac4\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Info_SuTalTac4,
    );
    menu_bar.add_emit(
        "Info/Licenza\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Info_Licenza,
    );
    menu_bar.add_emit(
        "Info/Manuale Utente\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Info_ManualeUtente,
    );
    menu_bar.add_emit(
        "Info/Tutorial\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Info_Tutorial,
    );
    menu_bar.add_emit(
        "Info/Supporto\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Info_Supporto,
    );
    menu_bar.end();
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
