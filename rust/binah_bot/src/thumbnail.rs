use std::error::Error;

use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::InvocationType;
use xxhash_rust::xxh3::xxh3_64;
use serde::Serialize;

// todo: figure out how to distribute this across packages without copy paste
#[derive(Debug, Serialize)]
struct ThumbnailLambdaInput {
    pub combat_pages: [String; 9],
    pub thumb_name: String
}

pub async fn generate_thumbnail(
    client: &aws_sdk_lambda::Client,
    function_name: &str,
    combat_pages: &[Option<String>; 9]
) ->  Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Generating thumbnail with combat_pages={:?}", combat_pages);
    let name = generate_thumb_name(&combat_pages);

    let blob = serde_json::to_string(&ThumbnailLambdaInput {
        combat_pages: resolve_optional(combat_pages),
        thumb_name: name
    })?;

    tracing::info!("Invoking generate thumbnail lambda with blob={}", blob);
    let response = client.invoke()
        .function_name(function_name)
        .invocation_type(InvocationType::Event)
        .payload(Blob::new(blob.as_bytes()))
        .send()
        .await?;
    tracing::info!("Received response status code={}", response.status_code());

    Ok(())
}

pub fn generate_thumb_name(
    combat_pages: &[Option<String>; 9]
) -> String {
    format!("{:X}", xxh3_64(&resolve_optional(combat_pages).join("#").as_bytes()))
}

fn resolve_optional(
    combat_pages: &[Option<String>; 9]
) -> [String; 9] {
    combat_pages.into_iter().map(|x| {
        x.clone().unwrap_or("0".to_string())
    }).collect::<Vec<_>>().try_into().expect("couldn't cast")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_generate_thumb_name() {
        let input = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"].into_iter()
            .map(|x| Some(x.to_string())).collect::<Vec<_>>().try_into().unwrap();
        let input2 = vec!["a", "b", "c", "d", "e", "f", "g", "h", "j"].into_iter()
            .map(|x| Some(x.to_string())).collect::<Vec<_>>().try_into().unwrap();

        assert_ne!(generate_thumb_name(&input), generate_thumb_name(&input2));
    }

}