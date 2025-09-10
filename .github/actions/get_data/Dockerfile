# action will be run in python3 container
FROM python:3@sha256:35c05b6d32b11a1a8134f35dd0db68ef6fbb21ce62934a7aec6d50977915a92c
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
