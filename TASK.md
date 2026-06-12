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

    url = f"https://index.commoncrawl.org/{index_name}-index"
    params = [
        ("url", pattern),
        ("output", "json"),
        ("filter", "status:200"),
        ("filter", "mime:text/html"),
        ("fl", "url,mime,status,timestamp,filename,offset,length,digest"),
        ("collapse", "digest"),
    ]
    requests.get(url, params=params, timeout=120, stream=True)

### crawls

Returns a list of available crawls.

https://index.commoncrawl.org/collinfo.json