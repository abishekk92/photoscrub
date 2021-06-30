### PhotoScrub

Scrub geotag and other metadata from your image files.

## Usage


# Default

Applies sane default, removes all personally identifiable information.

```
photoscrub <FILE> <OUTFILE>
````


# View all metadata

```
photoscrub <FILE> list (all|device|geo) -show
```


# Scrub all metadata

```
photoscrub <FILE> scrub (all|device|geo) <OUTFILE>
```


# Overwrite all metadata

```
photoscrub <FILE> overwrite (all|device|geo) <OUTFILE>
```
