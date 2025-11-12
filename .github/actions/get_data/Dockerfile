# action will be run in python3 container
FROM python:3@sha256:e6b1f7011589cc717a5112e6fdb56217e9e734a57e4cb50216e912b068b392a8
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
