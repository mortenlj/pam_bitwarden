# action will be run in python3 container
FROM python:3@sha256:3592650a0a8e6709cd4f3359dc7f1665c35bd9da6da34a9f22050aaf4878511f
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
