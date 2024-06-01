use maud::{html, Markup, PreEscaped};
use css_minify::optimizations::{Minifier, Level};
use rocket::response::Redirect;
use rocket::{get, routes, launch};
use rocket::fs::FileServer;

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static_unsafe {
	($path: expr) => (PreEscaped(include_static!($path)))
}

macro_rules! include_css {
	($path: expr) => (PreEscaped(Minifier::default().minify(include_static!($path), Level::Three).unwrap()))
}

fn base(content: Markup) -> Markup {
    html! {
        html lang="fr"{
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta charset="utf-8";
                meta name="robots" content="index, follow";
                meta name="theme-color" content="#000000";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="keywords" name="keywords" content="Discord, discord, epl, students, école, polytechnique, louvain, ingénieur, cercle, industriel, uclouvain";
                meta http-equiv="description" name="description" content="EPLStudents est un site recensant les liens utiles et les projets de la communauté étudiante de l'EPL.";
                meta http-equiv="cache-control" content="no-cache";
                meta http-equiv="pragma" content="no-cache";
				meta name="author" content="EPLStudents";
				meta name="og:title" content="EPLStudents";
				meta name="og:description" content="EPLStudents est un site recensant les liens utiles et les projets de la communauté étudiante de l'EPL.";
				meta name="og:image" content="https://epl-students.be/public/thumbnails/eplwiki.png";
                
                link rel="icon" href="/public/favicon.ico";

                style {
					(include_css!("/main.css"))
				}
                title { "EPLStudents" }
            }
            body {
                (content)
            }
        }
    }

}

fn project(title: &'static str, link: &'static str, img_src: &'static str, descr: Markup) -> Markup {
	let alt: &str = &(title.to_owned() + " thumbnail");

	html! {
		.project {
			.labeled-img {
				img alt=(alt) src=(img_src);
				div {
					div {
						h2 { (title) }
					}
				}
			}
			p { (descr) }
			a.learn-more href=(link) {
				p { "Voir" }
			}
		}
	}
}

fn button(handle: &'static str, link: &'static str, icon: PreEscaped<&str>) -> Markup {
	html! {
		a.button href=(link) {
			(icon)
			p { (handle) }
		}
	}
}

const EPLWIKI_IMG_SRC:   &str = "/public/thumbnails/eplwiki.png";
const DISCORD_IMG_SRC:   &str = "/public/thumbnails/discord.png";
const UCLOUVAINDOWN_IMG_SRC: &str = "/public/thumbnails/uclouvain-down.png";
const REVUE_IMG_SRC: &str = "/public/thumbnails/revue.png";

#[get("/")]
fn index() -> Markup {
	base(html! {
		.container {
			header {
				h1 { "EPLStudents" }
			}
            .separation {}
			main {
				p {
					"Ce site recense tous les projets inventés par la communauté étudiante de l'EPL. Vous y trouverez des projets de cours, des projets personnels, ou même des projets à destination de la communauté comme ce site 🙃."
				}
				.projects {
					(project("EPLWiki", "https://wiki.epl-students.be", EPLWIKI_IMG_SRC, html! {
						"Un recueil d'explications, de conseils et d'astuces sur la vie à l'EPL. "
					}))

					(project("Discord EPL", "https://discord.epl-students.be", DISCORD_IMG_SRC, html! {
						"Le Discord des étudiants ingénieurs de l'EPL. N'hésitez pas à nous rejoindre !"
					}))

					(project("UCLouvain Down", "https://www.uclouvain-down.be", UCLOUVAINDOWN_IMG_SRC, html! {
						"Un site permettant de vérifier le statut des différents services de l'UCLouvain."
					}))

					(project("La revue des Ingénieurs", "https://revuedesingenieurs.be/", REVUE_IMG_SRC, html! {
						"La seule et unique revue des ingénieurs ! Vous pourrez retrouver les chansons de la revue, des photos, des archives et bien plus encore !"
					}))
				}
			}

			footer {
                p { "Ce site est open-source et maintenu par la communauté étudiante de l'EPL." }
				.buttons {
					(button("Source code", "https://github.com/EPLStudents/eplstudents.github.io", include_static_unsafe!("/icons/gh.svg")))
				}
			}
		}
	})
}

#[get("/discord")]
fn discord() -> Redirect {
    Redirect::to("https://discord.gg/3ZH2YWhsCa")
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index, discord])
		.mount("/public", FileServer::from(relative!("/public")))
}