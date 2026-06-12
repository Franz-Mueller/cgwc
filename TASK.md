# Common Crawl Website Constructor

The Common Crawl Website Constructor (ccwc) is a tool that retrieves CommonCrawl Data in order to reconstruct websites. 

## Commands

### construct

Queries all results for a given URL in one or more given crawls.

#### Arguments

- **URL**: mandatory
- **CRAWLS**: can be one or a list of crawls to query. Without it it will query all
- **DATESPAN**: a date or a daterange for filtering crawls either crawl, datespan or both must be given
- **ZIP**: if true, the website will be packaged into a zip
- **IGNOREDUPLICATES**: if true there can be multiple versions of a path. If false the newest version will be used

#### Process



### crawls

Returns a list of available crawls.
