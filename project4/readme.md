This is individual project 4:  

# Serverless Data Engineering Pipeline

Step 0: Analyzing the Dataset

Wikiticker JSON Dataset

First we should understand the incoming rows of data from our /usr/hdp/3.0.1.0-187/druid/quickstart/wikiticker-2015-09-12-sampled.json.gz dataset.
```
{
    "time":"2015-09-12T00:47:05.474Z",
    "channel":"#en.wikipedia",
    "cityName":"Auburn",
    "comment":"/* Status of peremptory norms under international law */ fixed spelling of 'Wimbledon'",
    "countryIsoCode":"AU",
    "countryName":"Australia",
    "isAnonymous":true,
    "isMinor":false,
    "isNew":false,
    "isRobot":false,
    "isUnpatrolled":false,
    "metroCode":null,
    "namespace":"Main",
    "page":"Peremptory norm",
    "regionIsoCode":"NSW",
    "regionName":"New South Wales",
    "user":"60.225.66.142",
    "delta":0,
    "added":0,
    "deleted":0
}
```
Every row in our dataset will have the same keys as above with different values. Let's separate our timestamp (unique-identifier attribute), dimensions (String-typed attributes) and metrics (numeric-typed attributes) into their own groups:

timestamp

"time"
Timestamp can be found in the time field. If your dataset doesn't have a time field, you can tag all rows with either a fixed timestamp "2000-01-01T00:00:00.000Z" or you can insert the current time using your favorite programming language.

dimensions
```
  "channel",
  "cityName",
  "comment",
  "countryIsoCode",
  "countryName",
  "isAnonymous",
  "isMinor",
  "isNew",
  "isRobot",
  "isUnpatrolled",
  "metroCode",
  "namespace",
  "page",
  "regionIsoCode",
  "regionName",
  "user"
 ```
The above keys all have String-typed values

metrics
```
{
  "name" : "count",
  "type" : "count"
},
{
  "name" : "added",
  "type" : "longSum",
  "fieldName" : "added"
},
{
  "name" : "deleted",
  "type" : "longSum",
  "fieldName" : "deleted"
},
{
  "name" : "delta",
  "type" : "longSum",
  "fieldName" : "delta"
},
{
  "name" : "user_unique",
  "type" : "hyperUnique",
  "fieldName" : "user"
}
```
Some useful metrics to aggregate in regards to our dataset will be the total number of rows in the dataset, so the count. Another useful metric will be to aggregate or collect all the the added keys, then compute their sum using Druid's longSum Aggregator API. We can find the sum for the deleted keys, then for the delta keys. Another metric that we can collect is the user key since each row has their own unique user field. At index time, the unique user keys will be aggregated to hyperUnique metric set.

Now that we analyzed our dataset and separated into timestamp, dimensions and metrics groups, this information can help us in writing the Druid Ingestion Spec.



Step 1: Define the Data Pipeline

The first step is to define the data pipeline. For this example, let's consider a scenario where we need to extract data from social media, perform some natural language processing on the text data, and store the processed data in a database. The pipeline consists of the following steps:

Extract data from social media using an API.
Process the text data using natural language processing or computer vision.
Store the processed data in a database.
Step 2: Choose the Serverless Technologies

The next step is to choose the serverless technologies that will be used to implement the data pipeline. For this example, we will use the following serverless technologies:

AWS Lambda - to run our code in response to events.
AWS API Gateway - to create a REST API for our Lambda function.
AWS S3 - to store our raw data and the processed data.
AWS Glue - to transform the data and store it in a database.
Step 3: Implement the Data Pipeline

Now let's implement the data pipeline using the serverless technologies we've chosen.

Extract data from social media using an API - we can use AWS Lambda to create a function that will make calls to the social media API and retrieve data. We can store the raw data in an S3 bucket.
Process the text data using natural language processing or computer vision - for this example, let's say we want to perform sentiment analysis on the text data. We can use AWS Lambda to create a function that will take the raw text data from the S3 bucket, perform sentiment analysis using the AWS Comprehend service, and store the processed data in another S3 bucket.
Store the processed data in a database - we can use AWS Glue to transform the processed data and store it in a database, such as Amazon Redshift or Amazon Aurora.
Step 4: Extend the Functionality

To extend the functionality of our NLP analysis, we can add entity extraction or key phrase extraction. Here are the steps to implement entity extraction:

Modify the Lambda function that performs the NLP analysis to extract entities from the text data using AWS Comprehend.
Store the entities in a separate S3 bucket.
Use AWS Glue to transform the entity data and store it in the same database as the sentiment data.
To implement key phrase extraction, we can follow similar steps as entity extraction, but use the AWS Comprehend service to extract key phrases from the text data instead.

If we want to do Applied Computer Vision, we can use Amazon Rekognition to analyze images and videos. Here are the steps to implement computer vision:

Create an S3 bucket to store the raw images or videos.
Use AWS Lambda to trigger the Amazon Rekognition service to perform image or video analysis.
Store the processed data in another S3 bucket.
Use AWS Glue to transform the data and store it in the database.
These are just a few examples of how we can extend the functionality of our serverless data engineering pipeline using NLP or Computer Vision.
