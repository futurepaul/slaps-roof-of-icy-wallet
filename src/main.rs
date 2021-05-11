use std::fmt;

use iced::Clipboard;
use iced::Space;
use iced::{
    button, window, Application, Button, Column, Command, Container, Element, Length, Settings,
    Text,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use bdk::{
    self, bitcoin,
    blockchain::{noop_progress, ElectrumBlockchain},
    database::MemoryDatabase,
    electrum_client::Client,
    KeychainKind,
};

mod coldcard;
use coldcard::ColdcardJson;

mod components;
mod theme;

use components::my_button;
use theme::{BOLD, REGULAR};

async fn open_file() -> Result<ColdcardJson, LoadError> {
    use async_std::prelude::*;
    use async_std::task;

    // TODO: no idea if this is how to do async
    let open_file = task::block_on(async {
        let open_file: String;
        match tinyfiledialogs::open_file_dialog("Open", "coldcard-export.json", None) {
            Some(file) => open_file = file,
            None => open_file = "null".to_string(),
        }
        open_file
    });

    let path = PathBuf::from_str(&open_file).unwrap();

    let mut contents = String::new();

    let mut file = async_std::fs::File::open(path)
        .await
        .map_err(|_| LoadError::FileError)?;

    file.read_to_string(&mut contents)
        .await
        .map_err(|_| LoadError::FileError)?;

    let coldcard_json = ColdcardJson::from_str(&contents).map_err(|_| LoadError::ParseError)?;

    Ok(coldcard_json)
}
fn main() -> iced::Result {
    let mut settings = Settings::default();
    let mut window_settings = window::Settings::default();
    window_settings.size = (440, 400);
    settings.window = window_settings;
    IcedWallet::run(settings)
}

struct IcedWallet {
    routes: Routes,
}

#[derive(Debug, Clone)]
struct Routes {
    routes: Vec<Route>,
    current: usize,
}

#[derive(Clone)]
struct BdkWallet {
    wallet: Arc<bdk::Wallet<ElectrumBlockchain, MemoryDatabase>>,
}

impl BdkWallet {
    fn new(wallet: bdk::Wallet<ElectrumBlockchain, MemoryDatabase>) -> Self {
        Self {
            wallet: Arc::new(wallet),
        }
    }
}

impl fmt::Debug for BdkWallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BdkWAllet")
            .field("wallet", &"THIS IS THE WALLET")
            .finish()
    }
}

#[derive(Debug, Clone)]
enum Route {
    Setup {
        import_button: button::State,
    },
    ImportConfirm {
        coldcard_info: Option<ColdcardJson>,
        cancel_button: button::State,
        confirm_button: button::State,
    },
    Home {
        wallet: Option<BdkWallet>,
        sync_button: button::State,
    },
}

#[derive(Debug, Clone, Copy)]
enum RouteAlias {
    Setup,
    ImportConfirm,
    Home,
}

impl IcedWallet {
    fn create_wallet_from_coldcard(
        parsed_coldcard: ColdcardJson,
    ) -> Result<bdk::Wallet<ElectrumBlockchain, MemoryDatabase>, bdk::Error> {
        let client = Client::new("tcp://localhost:51401").expect("Couldn't make a client");

        let desc = parsed_coldcard.build_descriptor(false)?;

        dbg!(desc.clone());

        let _change_desc = parsed_coldcard.build_descriptor(true)?;

        let wallet = bdk::Wallet::new(
            desc,
            None,
            bitcoin::Network::Regtest,
            MemoryDatabase::default(),
            ElectrumBlockchain::from(client),
        )?;

        let descriptor = wallet
            .public_descriptor(KeychainKind::External)?
            .unwrap()
            .to_string();
        dbg!(descriptor);

        // println!("{:?}", wallet.get_new_address());
        wallet.sync(noop_progress(), None)?;

        println!("Descriptor balance: {} SAT", wallet.get_balance()?);

        Ok(wallet)
    }
}

impl<'a> Route {
    fn view(&mut self) -> Element<WalletMessage> {
        match self {
            Route::Setup { import_button } => Self::setup_view(import_button),
            Route::ImportConfirm {
                cancel_button,
                confirm_button,
                coldcard_info,
            } => Self::import_confirm_view(cancel_button, confirm_button, coldcard_info),
            Route::Home {
                sync_button,
                wallet,
            } => Self::home_view(sync_button, wallet),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, WalletMessage> {
        Column::new()
            .spacing(20)
            .width(440.into())
            .push(Text::new(title).font(BOLD).size(30))
    }

    fn setup_view(button_state: &'a mut button::State) -> Column<'a, WalletMessage> {
        let button_content = Column::new()
            .spacing(10)
            .padding(30)
            .push(Text::new("ICON"))
            .push(Text::new("Import coldcard json").font(REGULAR).size(18));

        let big_button = Button::new(button_state, button_content)
            .height(200.into())
            .width(200.into())
            .style(theme::Button)
            .on_press(WalletMessage::ImportPressed);

        let content = Container::new(big_button)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Self::container("Setup").push(content)
    }

    fn import_confirm_view(
        cancel_button_state: &'a mut button::State,
        confirm_button_state: &'a mut button::State,
        coldcard_info: &mut Option<ColdcardJson>,
    ) -> Column<'a, WalletMessage> {
        let coldcard = coldcard_info.as_ref().unwrap();

        let xfp = coldcard.xfp.clone();

        let cancel_button = my_button("Cancel", cancel_button_state, WalletMessage::CancelPressed);
        let confirm_button = my_button(
            "Confirm",
            confirm_button_state,
            WalletMessage::ConfirmPressed,
        );

        //let content = Column::new().push(Text::new(xfp)).push(Space).push(cancel_button)

        Self::container("How does this look?")
            .push(Text::new(xfp))
            .push(Space::with_height(Length::Fill))
            .push(cancel_button)
            .push(confirm_button)
    }

    fn home_view(
        sync_button_state: &'a mut button::State,
        wallet: &mut Option<BdkWallet>,
    ) -> Column<'a, WalletMessage> {
        // This should be safe because we should only be here when we have a wallet

        let wallet = wallet.clone().unwrap();
        let balance = wallet.wallet.get_balance().unwrap();

        // TODO: I'm recreating this wallet each time so I get the same address every time :(
        let address = wallet.wallet.get_new_address().unwrap();
        let sync_button = my_button("Sync Wallet", sync_button_state, WalletMessage::SyncPressed);
        println!("{}", address);
        Self::container("Wallet")
            .push(Text::new("THIS IS THE WALLET"))
            .push(Text::new(format!("Balance: {}", balance)))
            .push(Text::new(format!("Address: {}", address)))
            .push(sync_button)
    }
}

impl Routes {
    fn new() -> Self {
        Self {
            routes: vec![
                Route::Setup {
                    import_button: button::State::new(),
                },
                Route::ImportConfirm {
                    cancel_button: button::State::new(),
                    confirm_button: button::State::new(),
                    coldcard_info: None,
                },
                Route::Home {
                    sync_button: button::State::new(),
                    wallet: None,
                },
            ],
            current: 0,
        }
    }

    fn active_route(&mut self) -> Route {
        // This shouldn't panic because we should always have a legal index
        self.routes[self.current].clone()
    }

    fn view(&mut self) -> Element<WalletMessage> {
        self.routes[self.current].view()
    }

    fn set_route_state(&mut self, route: RouteAlias, new_state: Route) {
        let index = Self::get_index_from_alias(route);
        self.routes[index] = new_state;
    }

    fn get_index_from_alias(alias: RouteAlias) -> usize {
        match alias {
            RouteAlias::Setup => 0,
            RouteAlias::ImportConfirm => 1,
            RouteAlias::Home => 2,
        }
    }

    fn nav_to(&mut self, to_route: RouteAlias) {
        self.current = Self::get_index_from_alias(to_route);
    }
}

#[derive(Debug, Clone)]
enum WalletMessage {
    ImportPressed,
    ConfirmPressed,
    CancelPressed,
    SyncPressed,
    FileOpened(Result<ColdcardJson, LoadError>),
}

#[derive(Debug, Clone)]
enum LoadError {
    ParseError,
    FileError,
}

impl Application for IcedWallet {
    type Executor = iced::executor::Default;
    type Message = WalletMessage;
    type Flags = ();

    fn new(_flags: ()) -> (IcedWallet, Command<WalletMessage>) {
        (
            IcedWallet {
                routes: Routes::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Is this the window title?")
    }

    fn update(
        &mut self,
        message: WalletMessage,
        _clipboard: &mut Clipboard,
    ) -> Command<WalletMessage> {
        match self.routes.active_route() {
            Route::Setup { .. } => match message {
                WalletMessage::ImportPressed => {
                    println!("Import pressed");
                    return Command::perform(open_file(), WalletMessage::FileOpened);
                }

                WalletMessage::FileOpened(result) => {
                    match result {
                        Ok(coldcard_info) => {
                            self.routes.set_route_state(
                                RouteAlias::ImportConfirm,
                                Route::ImportConfirm {
                                    cancel_button: button::State::new(),
                                    confirm_button: button::State::new(),
                                    coldcard_info: Some(coldcard_info),
                                },
                            );

                            self.routes.nav_to(RouteAlias::ImportConfirm);
                        }
                        Err(_) => panic!("Oh no!"),
                    }
                    //println!("{:?}", path);
                }
                _ => {
                    panic!("There shouldn't be any other buttons on this screen!");
                }
            },
            Route::ImportConfirm { coldcard_info, .. } => match message {
                WalletMessage::CancelPressed => self.routes.nav_to(RouteAlias::Setup),
                WalletMessage::ConfirmPressed => {
                    let coldcard = coldcard_info.unwrap();
                    let wallet = Self::create_wallet_from_coldcard(coldcard);
                    match wallet {
                        Ok(wallet) => {
                            self.routes.set_route_state(
                                RouteAlias::Home,
                                Route::Home {
                                    sync_button: button::State::new(),
                                    wallet: Some(BdkWallet::new(wallet)),
                                },
                            );

                            self.routes.nav_to(RouteAlias::Home)
                        }
                        Err(e) => eprintln!("Oh no! {}", e),
                    }
                }
                _ => {
                    panic!("There shouldn't be any other buttons on this screen!");
                }
            },
            Route::Home { wallet, .. } => match message {
                WalletMessage::SyncPressed => {
                    let new_wallet = wallet.unwrap();
                    new_wallet.wallet.sync(noop_progress(), None).unwrap();
                    self.routes.set_route_state(
                        RouteAlias::Home,
                        Route::Home {
                            sync_button: button::State::new(),
                            wallet: Some(new_wallet),
                        },
                    );
                }
                _ => {
                    panic!("There shouldn't be any other buttons on this screen!");
                }
            },
        }
        Command::none()
    }
    fn view(&mut self) -> Element<WalletMessage> {
        let active_route = self.routes.view();

        let content: Element<_> = Column::new()
            .max_width(440)
            .spacing(20)
            .padding(20)
            .push(active_route)
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Basic)
            .into()
    }
}
