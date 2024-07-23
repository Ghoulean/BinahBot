mod models;

use std::env;
use std::error::Error;
use std::io::Cursor;

use futures::stream;
use futures::StreamExt;
use image::GenericImage;
use image::ImageFormat;
use image::ImageReader;
use image::RgbaImage;
use lambda_runtime::run;
use lambda_runtime::service_fn;
use lambda_runtime::LambdaEvent;
use models::ThumbnailGeneratorEnvironment;
use models::ThumbnailLambdaInput;
use ruina::ruina_reparser::get_combat_page_by_id;
use aws_sdk_s3::primitives::ByteStream;

static NOT_FOUND_IMAGE_NAME: &str = "404_Not_Found";
static BASE_COMBAT_PAGE_WIDTH: u32 = 410;
static BASE_COMBAT_PAGE_HEIGHT: u32 = 310;

async fn function_handler(
    event: ThumbnailLambdaInput,
    env: &ThumbnailGeneratorEnvironment,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::debug!("Rust function invoked");

    let thumbnail_key = format!("{}/{}.png", env.s3_directory, event.thumb_name);
    let head_object_result = env.s3_client.as_ref().unwrap().head_object()
        .bucket(&env.s3_bucket_name)
        .key(&thumbnail_key)
        .send()
        .await;

    // There is a TOCTOU gap here, but since put doesn't affect the
    // correctness of this logic, this is only here as an optimization
    if head_object_result.is_ok() {
        tracing::info!("Head Object resulted in success; no need to generate thumbnail");
        return Ok(());
    }

    let image_dirs = event.combat_pages.into_iter().map(|x| {
        get_combat_page_by_id(&x)
    }).map(|x| {
        format!("{}.png", x.and_then(|y| y.artwork).unwrap_or(NOT_FOUND_IMAGE_NAME))
    }).collect::<Vec<_>>();

    tracing::info!("Grabbing images: {:?}", image_dirs);

    // todo: do not make redundant requests on duplicates
    let get_object_data = stream::iter(image_dirs).map(|x| async {
        env.s3_client.as_ref().unwrap().get_object()
            .bucket(&env.s3_bucket_name)
            .key(x)
            .send()
            .await
            .expect("request failed")
    }).buffered(9).map(|x| async {
        x.body.collect().await.expect("couldn't get image data")
    }).buffered(9).collect::<Vec<_>>().await;

    let image_data = get_object_data.into_iter().map(|x| {
        ImageReader::with_format(
            std::io::Cursor::new(x.into_bytes()),
            ImageFormat::Png
        ).decode().expect("couldn't decode image")
    }).collect::<Vec<_>>();

    tracing::info!("Drawing canvas");

    let mut canvas = RgbaImage::new(BASE_COMBAT_PAGE_WIDTH * 3, BASE_COMBAT_PAGE_HEIGHT * 3);
    for i in 0..=2 {
        for j in 0..=2 {
            let index = i * 3 + j;
            let image = &image_data[index];
            let resized = image.resize_exact(
                BASE_COMBAT_PAGE_WIDTH,
                BASE_COMBAT_PAGE_HEIGHT,
                image::imageops::FilterType::Lanczos3
            );

            let _ = canvas.copy_from(&resized, j as u32 * BASE_COMBAT_PAGE_WIDTH, i as u32 * BASE_COMBAT_PAGE_HEIGHT);
        }
    }

    tracing::info!("Converting to PNG");

    let mut bytes: Vec<u8> = Vec::new();
    canvas.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;

    tracing::info!("Putting to S3");

    let _ = env.s3_client.as_ref().unwrap().put_object()
        .bucket(&env.s3_bucket_name)
        .key(&thumbnail_key)
        .body(ByteStream::from(bytes))
        .content_type("image/png")
        .send()
        .await?;

    tracing::debug!("Rust function finished invocation");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let config = aws_config::load_from_env().await;
    let s3 = aws_sdk_s3::Client::new(&config);

    let env = ThumbnailGeneratorEnvironment {
        s3_bucket_name: env::var("S3_BUCKET_NAME").unwrap(),
        s3_directory: env::var("S3_DIRECTORY").unwrap(),
        s3_client: Some(s3)
    };
    let env_ref = &env;

    tracing::debug!("Rust function setup complete");

    run(service_fn(move |event: LambdaEvent<ThumbnailLambdaInput>| {
        function_handler(event.payload, env_ref)
    })).await
}

#[cfg(test)]
mod tests {
    use crate::ThumbnailGeneratorEnvironment;
    use crate::ThumbnailLambdaInput;
    use crate::function_handler;

    // todo: this test is broken. figure out how to run tests
    // using SSO crednetials provider
    #[ignore]
    #[tokio::test]
    async fn sanity() {
        let config = aws_config::load_from_env().await;
        let config = aws_sdk_s3::config::Builder::from(&config)
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .build();
        let s3 = aws_sdk_s3::Client::from_conf(config);
        let env = ThumbnailGeneratorEnvironment {
            s3_bucket_name: "".to_string(),
            s3_directory: "deck_thumbnails".to_string(),
            s3_client: Some(s3)
        };
        let input = ThumbnailLambdaInput {
            combat_pages: vec![
                "608014","608014","608014","608015","608015","608015","608009","608009","608004"
            ].into_iter().map(|x| x.to_string()).collect::<Vec<_>>().try_into().unwrap(),
            thumb_name: "turbo_nikolai_test".to_string()
        };

        assert!(function_handler(input, &env).await.is_ok());
    }
}
