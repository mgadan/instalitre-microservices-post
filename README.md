## Install rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## .env
    
    DATABASE_URL_TEST=postgres://user:password@address/name_database
    accesskey= scaleway > api token > generate new token (access key)
    secretkey= scaleway > api token > generate new token (your ornisation id)
    nameBucket= scaleway > object storage > create bucket > bucket name
    region=fr-par
    endpoint=https://s3.fr-par.scw.cloud
    port=numero de port
    adresse est localhost

## run api
    cargo run
    
## Routes
    
    /post/ 
        POST: add post
    
    /post/{id} 
        GET: get post with post uuid
        DELETE: delete post with post uuid
        PUT: put post with post uuid
    
    /post/user/{id} 
        GET: get all post with user uuid
        DELETE: delete all post with uuid user
    
    /image/{author}/{post}
        GET: return image in base 64 with uuid author and uuid photo
       
## POSTMAN requete:
    https://www.getpostman.com/collections/c0e2007560ad56852e67
