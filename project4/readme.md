This is individual project 4:  

# Serverless Data Engineering Pipeline

## Step 0: Analyzing the Dataset

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



## Step 1: Define the Data Pipeline

The first step is to define the data pipeline. For this example, let's consider a scenario where we need to extract data from social media, perform some natural language processing on the text data, and store the processed data in a database. The pipeline consists of the following steps:

Extract data from social media using an API.
Process the text data using natural language processing or computer vision.
Store the processed data in a database.



Open the AWS Management Console and navigate to the S3 service.
Click the "Create bucket" button.
Enter a unique name for your bucket. Bucket names must be unique across all existing bucket names in Amazon S3. You can choose any name as long as it meets the following requirements:
Bucket names must be at least 3 and no more than 63 characters long.
Bucket names can contain lowercase letters, numbers, and hyphens.
Bucket names cannot begin or end with a hyphen.
Bucket names cannot be formatted as an IP address (e.g., 192.168.5.4).
Choose the region for your bucket. It's recommended to choose the region closest to you or where you expect most of your users to be located.
Leave the "Block all public access" option enabled to ensure that the bucket and its contents are not accessible to the public. You can modify the access settings later if needed.
Click the "Create bucket" button to create your bucket.
Once you have created your S3 bucket, you can start uploading objects (files) to it, configure access permissions, and integrate it with other AWS services.

## Step 2: Choose the Serverless Technologies

The next step is to choose the serverless technologies that will be used to implement the data pipeline. For this example, we will use the following serverless technologies:

AWS Lambda - to run our code in response to events.
AWS API Gateway - to create a REST API for our Lambda function.
AWS S3 - to store our raw data and the processed data.
AWS Glue - to transform the data and store it in a database.  



Open the AWS Management Console and navigate to the S3 service.
Click on the name of the bucket you created earlier.
Click the "Create folder" button.
Enter "input" as the folder name and click the "Create" button. This folder will be used to store the input data for your pipeline.
Repeat steps 3-4 to create two more folders named "code" and "output". The "code" folder will store your pipeline code, while the "output" folder will store the processed data generated by your pipeline.
Once you have created the folders, you can upload files to the "input" folder, store your code in the "code" folder, and configure your pipeline to write output data to the "output" folder.
## Step 3: Implement the Data Pipeline

Now let's implement the data pipeline using the serverless technologies we've chosen.

Extract data from social media using an API - we can use AWS Lambda to create a function that will make calls to the social media API and retrieve data. We can store the raw data in an S3 bucket.
Process the text data using natural language processing or computer vision - for this example, let's say we want to perform sentiment analysis on the text data. We can use AWS Lambda to create a function that will take the raw text data from the S3 bucket, perform sentiment analysis using the AWS Comprehend service, and store the processed data in another S3 bucket.
Store the processed data in a database - we can use AWS Glue to transform the processed data and store it in a database, such as Amazon Redshift or Amazon Aurora.


Open the AWS Management Console and navigate to the EMR service.
Click the "Create cluster" button.
On the "Quick options" page, choose the "Go to advanced options" link.
On the "Software and Steps" page, select the "emr-6.3.0" release and choose the "Spark" application. You can also choose other applications based on your use case.
In the "Hardware configuration" section, choose the instance type, number of instances, and other hardware settings for your cluster. You can also choose to use Spot instances to save costs.
In the "General cluster settings" section, enter a name for your cluster and choose the EC2 key pair that you want to use to access the instances in your cluster.
In the "Security and access" section, choose the VPC and subnet for your cluster. You can also specify additional security groups and IAM roles.
Optionally, you can configure additional settings such as bootstrap actions, steps, and debugging options.
Review the settings and click the "Create cluster" button to create your EMR cluster.


Connect to your EMR cluster using SSH. You can use your EC2 key pair to access the instances in your cluster. For example, if you are using a Mac or Linux machine, open the Terminal app and run the following command:
```
ssh -i <path_to_your_ec2_key_pair_file> hadoop@<your_master_node_public_dns_name>
```
Replace <path_to_your_ec2_key_pair_file> with the path to your EC2 key pair file, and <your_master_node_public_dns_name> with the public DNS name of the master node in your EMR cluster.
Once you are connected to the master node of your EMR cluster, create a new directory to store your PySpark code:

```
mkdir code
```
Copy your PySpark code to the new directory. You can use any text editor to write your PySpark code on your local machine, and then use the scp command to copy the file to your EMR cluster. For example, if you have a file named agg_filter.py in the ~/Documents directory on your local machine, you can run the following command to copy it to the code directory in your EMR cluster:
```
scp -i <path_to_your_ec2_key_pair_file> ~/Documents/agg_filter.py hadoop@<your_master_node_public_dns_name>:~/code/
```
Once your PySpark code is copied to the code directory in your EMR cluster, you can submit the PySpark job using the following command:
```
sudo spark-submit ~/code/agg_filter.py
```
This will run the PySpark job using the Spark application that you selected when creating the EMR cluster.
## Step 4: Extend the Functionality

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
