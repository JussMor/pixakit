# Pixakit

<table>
  <tr>
    <td width=300>
      <img src="docs/images/pixakit.png" alt="Pixakit" width="300" height="200">
    </td>
    <td>
      <p>Pixakit is a Open Source Project that give you the facility to have Image Resizing on the fly using your favority Cloud Provider (Azure, Amazon, Google Cloud) or self-hosting your images.</p>
      <p>The images are getting from your provider and cache using  <a href="https://github.com/moka-rs/moka">Moka Rs</a></p>
    </td>
  </tr>
</table>

# Philosophy

KISS -> Keep Stupid Simple.

Pixakit can be used with your current setup, just creating a wrapping function to transform your current provider url 
to a  valid pixakit url. 

Example:
  https://company.com/Home_Illustration_a2ba90ea37/Home_Illustration_a2ba90ea37.webp

  https://pixakit.com/api/v1/googlecloud/images/{bucket}/Home_Illustration_a2ba90ea37.webp

# Features

### Ondisk
- GET  = /api/v1/ondisk/get-all-files
- GET  = /api/v1/ondisk/get-files-and-folders
- GET  = /api/v1/ondisk/images/folder1/subfolder1/sebasback.webp?width=300&height=500
- POST = /api/v1/ondisk/upload?path=folder1/subfolder1

### Amazon S3

- GET  = /api/v1/amazon/images/pixakit/permissionless.webp?width=200&height=100

### Azure
- POST = /api/v1/azure/upload
- GET  = /api/v1/azure/images/pixakit/descentral.webp?width=500&height=300

### Google Cloud
- GET  = /api/v1/googlecloud/images/pixakit/Payment-Model.webp?width=500&height=300

You can find a valid Postman Collection file in:  [Postman](docs/postman).

## Enviroment Variables

### Ondisk 
STORAGE_PROVIDER = ONDISK

### Azure
STORAGE_ACCOUNT =

STORAGE_ACCESS_KEY =

### Aws
AWS_ACCESS_KEY =

AWS_SECRET_ACCESS_KEY =

### Google Cloud
Rename the file pixakit-key.json.example to pixakit-key.json and update with your google cloud credentials


# Futures 

- Add UI to manage assets using Astro.
- Simplify API where query params can be reused.
- Add support to upload files to cloud providers. 
- Limit the file types to be uploaded. 
- Add support to download files with the params. 
- Listing the files in caches with option to deleted or add a TIME_TO_LIVE.