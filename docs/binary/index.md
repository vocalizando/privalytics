## Binary save
Privalytics uses internally [Bincode](https://github.com/bincode-org/bincode) to serialize the analytics data into
binary.

Right now the format is not stable, because the `AnalyticsData` struct is going to have backwards-incompatible changes.

An example file is available at [example-doc.plytics.bin](example-doc.plytics.bin)
