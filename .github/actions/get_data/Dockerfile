# action will be run in python3 container
FROM python:3@sha256:d59d263333c08481fb66062240b3c34cdd8d215dcb3e9336a13e72cf034a184e
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
