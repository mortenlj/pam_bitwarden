# action will be run in python3 container
FROM python:3@sha256:3efe6d5302c6131cbfbdb089c0dff7cf5a85ae5675c025df8488da10010acced
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
