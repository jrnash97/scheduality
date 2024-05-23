use crate::utils::*;
use chrono::NaiveDate;
use poise::serenity_prelude::{ActionRowComponent, InputText, ModalInteractionData};

#[derive(poise::Modal)]
#[name = "Add Release"]
pub(crate) struct AddRelease {
    #[name = "Artist"]
    #[placeholder = "Artist Name"]
    pub artist: String,
    #[name = "Release Title"]
    #[placeholder = "Release Title"]
    pub name: String,
    #[name = "Label"]
    #[placeholder = "Label Name"]
    pub label: Option<String>,
    #[name = "Release Date (yyyymmdd)"]
    #[placeholder = "YYYYMMDD"]
    #[min_length = 8]
    #[max_length = 8]
    pub release_date: String,
}

#[derive(Debug)]
pub(crate) struct ReleaseSubmission {
    pub artist: String,
    pub name: String,
    pub label: Option<String>,
    pub release_date: NaiveDate,
}

impl ReleaseSubmission {
    pub fn from_modal_response(
        modal_response: ModalInteractionData,
    ) -> Result<ReleaseSubmission, Error> {
        let mut rows: Vec<Option<InputText>> = modal_response
            .components
            .into_iter()
            .map(|row| {
                if let Some(ActionRowComponent::InputText(component)) = row.components.get(0) {
                    Some(component.to_owned())
                } else {
                    None
                }
            })
            .collect();

        let artist = rows
            .iter_mut()
            .filter_map(|row| {
                if let Some(input_text) = row {
                    match input_text.custom_id.as_str() {
                        "artist" => input_text.value.take(),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .get(0)
            .ok_or(Error::from("Error getting 'Artist'"))?
            .to_owned();

        let name = rows
            .iter_mut()
            .filter_map(|row| {
                if let Some(input_text) = row {
                    match input_text.custom_id.as_str() {
                        "name" => input_text.value.take(),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .get(0)
            .ok_or(Error::from("Error getting 'Album'"))?
            .to_owned();

        let release_date = rows
            .iter_mut()
            .filter_map(|row| {
                if let Some(input_text) = row {
                    match input_text.custom_id.as_str() {
                        "release_date" => input_text.value.take(),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .get(0)
            .ok_or(Error::from("Error getting 'Release Date'"))?
            .to_owned();

        let label = rows
            .iter_mut()
            .filter_map(|row| {
                if let Some(input_text) = row {
                    match input_text.custom_id.as_str() {
                        "label" => input_text.value.take(),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .get(0)
            .cloned();

        Ok(ReleaseSubmissionBuilder::new()
            .artist(artist)
            .name(name)
            .label(label)
            .release_date(release_date)?
            .build()?)
    }
}

impl Default for ReleaseSubmission {
    fn default() -> Self {
        Self {
            artist: "Artist Name".to_string(),
            name: "Release Title".to_string(),
            label: None,
            release_date: NaiveDate::from_ymd_opt(0, 1, 1).unwrap(),
        }
    }
}

trait ArtistState {}

struct NoArtist;
struct Artist(String);

impl ArtistState for NoArtist {}
impl ArtistState for Artist {}

trait AlbumState {}

struct NoAlbum;
struct Album(String);

impl AlbumState for NoAlbum {}
impl AlbumState for Album {}

trait ReleaseDateState {}

struct NoReleaseDate;
struct ReleaseDate(NaiveDate);

impl ReleaseDateState for NoReleaseDate {}
impl ReleaseDateState for ReleaseDate {}

struct ReleaseSubmissionBuilder<
    TArtist: ArtistState,
    TAlbum: AlbumState,
    TReleaseDate: ReleaseDateState,
> {
    artist: TArtist,
    name: TAlbum,
    label: Option<String>,
    release_date: TReleaseDate,
}

impl ReleaseSubmissionBuilder<NoArtist, NoAlbum, NoReleaseDate> {
    fn new() -> Self {
        Self::default()
    }
}

impl<TArtist, TAlbum, TReleaseDate> ReleaseSubmissionBuilder<TArtist, TAlbum, TReleaseDate>
where
    TArtist: ArtistState,
    TAlbum: AlbumState,
    TReleaseDate: ReleaseDateState,
{
    fn artist(self, artist: String) -> ReleaseSubmissionBuilder<Artist, TAlbum, TReleaseDate> {
        ReleaseSubmissionBuilder {
            artist: Artist(artist),
            name: self.name,
            label: self.label,
            release_date: self.release_date,
        }
    }

    fn name(self, name: String) -> ReleaseSubmissionBuilder<TArtist, Album, TReleaseDate> {
        ReleaseSubmissionBuilder {
            artist: self.artist,
            name: Album(name),
            label: self.label,
            release_date: self.release_date,
        }
    }

    fn label(self, label: Option<String>) -> Self {
        Self { label, ..self }
    }

    fn release_date(
        self,
        release_date: String,
    ) -> Result<ReleaseSubmissionBuilder<TArtist, TAlbum, ReleaseDate>, Error> {
        Ok(ReleaseSubmissionBuilder {
            artist: self.artist,
            name: self.name,
            label: self.label,
            release_date: ReleaseDate(Self::string_to_date(&release_date)?),
        })
    }

    fn string_to_date(date_string: &str) -> Result<NaiveDate, Error> {
        if date_string.len() != 8 {
            return Err(Error::from("Invalid Date"));
        }
        NaiveDate::from_ymd_opt(
            date_string[..4]
                .parse()
                .map_err(|_| Error::from("Invalid Date"))?,
            date_string[4..6]
                .parse()
                .map_err(|_| Error::from("Invalid Date"))?,
            date_string[6..]
                .parse()
                .map_err(|_| Error::from("Invalid Date"))?,
        )
        .ok_or(Error::from("Invalid Date"))
    }
}

impl ReleaseSubmissionBuilder<Artist, Album, ReleaseDate> {
    fn build(self) -> Result<ReleaseSubmission, Error> {
        Ok(ReleaseSubmission {
            artist: self.artist.0,
            name: self.name.0,
            label: self.label,
            release_date: self.release_date.0,
        })
    }
}

impl Default for ReleaseSubmissionBuilder<NoArtist, NoAlbum, NoReleaseDate> {
    fn default() -> Self {
        Self {
            artist: NoArtist,
            name: NoAlbum,
            label: None,
            release_date: NoReleaseDate,
        }
    }
}
