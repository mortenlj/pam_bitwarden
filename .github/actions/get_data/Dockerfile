# action will be run in python3 container
FROM python:3@sha256:dd82a2be8703eec3824c8c5f693aa9836fb984f72a14b8a32502236d94c0f8cb
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
